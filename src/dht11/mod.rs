use defmt::Debug2Format;
use embassy_rp::gpio::{Flex, Level, Pull};
use embassy_time::Timer;
use crate::{DhtResources, SensorData, SensorSender};

const PULSE_TIMEOUT: u32 = u32::MAX;

#[derive(Debug)]
enum Error {
    InvalidChecksum,
    Timeout,
}

#[embassy_executor::task]
pub async fn dht_11_task(
    resources: DhtResources,
    sender: SensorSender,
) {
    let mut data = Flex::new(resources.pin);

    trace!("Awaiting DHT startup...");

    loop {
        let sensor_data = match read_data(&mut data).await {
            Ok(v) => v,
            Err(e) => { 
                error!("DHT failure: {}", Debug2Format(&e)); 
                Timer::after_secs(5).await;
                continue 
            },
        };
        trace!("Sensor data retrieved");
        
        let mut temperature = sensor_data[2] as f32;
        if sensor_data[3] & 0x80 != 0 { temperature = -1.0 - temperature };
        temperature += (sensor_data[3] & 0x0F) as f32 * 0.1;
        
        let humidity = sensor_data[0] as f32 + (sensor_data[1] as f32 * 0.1);
        
        let sending = SensorData { temperature, humidity: Some(humidity), pressure: None };
        debug!("DHT sensor result: {}", Debug2Format(&sending));
        
        sender.send(sending);
        Timer::after_secs(2).await;
    }
}

async fn read_data(pin: &mut Flex<'static>) -> Result<[u8; 4], Error> {
    if !init_pin_for_read(pin).await { return Err(Error::Timeout) };
    trace!("DHT Sensor responded!");
    
    let mut sensor_data = [0; 4];
    for b in sensor_data.iter_mut() {
        *b = read_byte(pin).await?;
    }
    let checksum = read_byte(pin).await?;
    
    trace!("Raw DHT sensor data: {:#x} {:#x}", sensor_data, checksum);
    
    let sensor_data_checksum = sensor_data.iter().fold(0u8, |l, r| l.wrapping_add(*r));
    if checksum != sensor_data_checksum {
        error!("Checksum failure for DHT: {:#x} vs {:#x}", checksum, sensor_data_checksum);
        Err(Error::InvalidChecksum)
    } else { Ok(sensor_data) }
}

async fn init_pin_for_read(pin: &mut Flex<'static>) -> bool {
    pin.set_as_input();
    pin.set_pull(Pull::Up);
    Timer::after_millis(1).await;

    pin.set_as_output();
    pin.set_low();
    Timer::after_millis(20).await;

    pin.set_as_input();
    pin.set_pull(Pull::Up);
    Timer::after_micros(55).await;

    if catch_pulse(pin, Level::Low) == PULSE_TIMEOUT {
        error!("DHT Sensor never pulled low!");
        return false;
    }
    if catch_pulse(pin, Level::High) == PULSE_TIMEOUT {
        error!("DHT Sensor never pulled high!");
        return false;
    }
    
    true
}

fn catch_pulse(pin: &Flex<'static>, level: Level) -> u32 {
    let mut count = 0;
    
    while pin.get_level() == level {
        count += 1;
        if count >= PULSE_TIMEOUT { return PULSE_TIMEOUT }
    }
    
    count
}

async fn read_byte(pin: &Flex<'static>) -> Result<u8, Error> {
    let mut byte = 0;
    for i in 0..8 {
        let mask = 1 << (7 - i);
        if read_bit(pin).await? {
            byte |= mask;
        }
    }
    
    Ok(byte)
}

async fn read_bit(pin: &Flex<'static>) -> Result<bool, Error> {
    conditional_read_pulse(|| pin.is_high()).await?;
    Timer::after_micros(35).await;
    let v = pin.is_high();
    conditional_read_pulse(|| pin.is_low()).await?;
    
    Ok(v)
}

async fn conditional_read_pulse<F: Fn() -> bool>(f: F) -> Result<(), Error> {
    for _ in 0..1000 {
        if f() { return Ok(()) }
        Timer::after_micros(1).await;
    }
    Err(Error::Timeout)
}