#![no_std]
#![no_main]

mod mpl115a2;
mod dht11;
mod sh1106;
mod ws2812;
mod helpers;
mod generated;
mod data_structs;

#[macro_use]
extern crate defmt;

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Watch;

pub(crate) use generated::*;
pub(crate) use data_structs::*;

static SENSOR_WATCH: Watch<CriticalSectionRawMutex, SensorData, 5> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let resources = split_resources!(peripherals);

    unwrap!(spawner.spawn(mpl115a2::mpl_i2c_task(resources.mpl115a2, SENSOR_WATCH.sender())));
    unwrap!(spawner.spawn(dht11::dht_11_task(resources.dht11, SENSOR_WATCH.sender())));
    unwrap!(spawner.spawn(sh1106::display_task(resources.sh1106, SENSOR_WATCH.receiver().unwrap())));
    unwrap!(spawner.spawn(ws2812::led_task(resources.ws2812, SENSOR_WATCH.receiver().unwrap())));
}
