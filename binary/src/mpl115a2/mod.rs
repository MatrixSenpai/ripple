mod helpers;
mod data_structs;
mod macros;

use defmt::Debug2Format;
use embassy_rp::i2c::{I2c, Config};
use embassy_time::Timer;
use crate::{Irqs, MplResources, SensorData, SensorSender};

use data_structs::*;

use macros::*;
use helpers::*;

#[embassy_executor::task]
pub async fn mpl_i2c_task(
    resources: MplResources,
    channel: SensorSender
) {
    let mut i2c_handle = I2c::new_async(
        resources.controller, resources.scl, resources.sda, Irqs, Config::default()
    );
    
    trace!("Awaiting i2c MPL sensor startup");
    Timer::after_millis(10).await;

    trace!("Fetching coefficient data for MPL Sensor");
    let cfs = initialize_temp_i2c(&mut i2c_handle).await;
    info!("Coefficient data fetched: {:?}", Debug2Format(&cfs));

    loop {
        trace!("Sending CONVERT and waiting...");
        i2c_handle.write_async(MPL_ADDR, [CONVERT, 0]).await.unwrap();
        Timer::after_millis(5).await;

        trace!("Getting temp data...");
        let (temperature, pressure) = get_i2c_temp_data(&mut i2c_handle, cfs).await;
        let result = SensorData { temperature, pressure: Some(pressure), humidity: None };

        debug!("{}", Debug2Format(&result));

        channel.send(result);

        Timer::after_millis(50).await;
    }
}

