mod pio;
mod helpers;

use embassy_rp::pio::Pio;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Receiver;
use smart_leds::RGBA;

use crate::{Irqs, SensorData, WsResources, helpers::rotate_calculate_average};
use pio::*;
use helpers::*;

const NUM_LEDS: usize = 24;
const BUFFER_LEN: usize = 50;
const HIGH_TEMP: f32 = 35.0;

#[embassy_executor::task]
pub async fn led_task(
    resources: WsResources,
    mut mpl_receiver: Receiver<'static, CriticalSectionRawMutex, SensorData, 5>,
) {
    let Pio { mut common, sm0, .. } = Pio::new(resources.controller, Irqs);
    let program = CustomPioWsProgram::new(&mut common);
    let mut ws2812 = CustomPioWs::<_, 0, NUM_LEDS>::new(&mut common, sm0, resources.dma, resources.data, &program);
    
    let mut temp_avg_buffer = [0.0f32; BUFFER_LEN];
    let mut hum_avg_buffer = [0.0f32; BUFFER_LEN];
    
    loop {
        let sensor_data = mpl_receiver.changed().await;
        let tmp_avg = rotate_calculate_average(&mut temp_avg_buffer, Some(sensor_data.temperature));
        let hum_avg = rotate_calculate_average(&mut hum_avg_buffer, sensor_data.humidity);
        
        let mut pixel_data = [RGBA::default(); NUM_LEDS];
        calculate_led_data(&mut pixel_data, tmp_avg);
        calculate_humidity_data(&mut pixel_data, hum_avg);

        ws2812.write(&pixel_data).await;
    }
}