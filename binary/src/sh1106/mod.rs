mod data_structs;
mod helpers;

use embassy_rp::gpio::{Level, Output};
use embassy_rp::spi::{Config, Spi};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Receiver;
use embassy_time::Timer;
use sh1106_driver::{Builder, prelude::*};
use sh1106_driver::prelude::DisplayRotation::Rotate180;
use sh1106_driver::prelude::DisplaySize::Display128x64;
use heapless::{String, Vec};

use crate::{helpers::*, SensorData, ShResources};
use data_structs::{ItemLocation};
use helpers::*;

const TEMP_BUFFER_LEN: usize = 50;

#[embassy_executor::task]
pub async fn display_task(
    resources: ShResources,
    mut mpl_receiver: Receiver<'static, CriticalSectionRawMutex, SensorData, 5>,
) {
    let data_handle = Output::new(resources.dc, Level::High);
    let chip_handle = Output::new(resources.cs, Level::High);
    let spi_handle = Spi::new_blocking_txonly(
        resources.controller, resources.clk, resources.miso, Config::default()
    );

    let mut display: GraphicsMode<_> = Builder::new()
        .with_rotation(Rotate180)
        .with_size(Display128x64)
        .connect_spi(spi_handle, data_handle, chip_handle)
        .into();

    display.init().unwrap();
    display.flush().unwrap();
    trace!("Display initialized");
    
    let mut temp_avg_buffer = [0.0f32; TEMP_BUFFER_LEN];
    
    draw_icons(&mut display);
    trace!("Initial icons drawn");
    
    
    loop {
        use core::fmt::write;
        let sensor_data = mpl_receiver.changed().await;
        
        let tmp_avg = rotate_calculate_average(&mut temp_avg_buffer, Some(sensor_data.temperature));
        
        let mut t_val = String::<12>::new();
        write(&mut t_val, format_args!("{}°C", rounded_for_display(tmp_avg))).unwrap();
        write_text(&mut display, ItemLocation::Temperature, t_val);
        
        if let Some(pressure) = sensor_data.pressure {
            let mut p_val = String::<12>::new();
            write(&mut p_val, format_args!("{} kPa", rounded_for_display(pressure))).unwrap();
            write_text(&mut display, ItemLocation::Pressure, p_val);
        }
        
        if let Some(humidity) = sensor_data.humidity {
            let mut p_val = String::<12>::new();
            write(&mut p_val, format_args!("{}%", rounded_for_display(humidity))).unwrap();
            write_text(&mut display, ItemLocation::Humidity, p_val);
        }

        display.flush().unwrap();
        
        Timer::after_millis(500).await;
    }
}
