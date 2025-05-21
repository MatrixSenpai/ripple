use embassy_rp::rom_data::float_funcs::float_to_uint;
use smart_leds::RGBA;
use crate::ws2812::{HIGH_TEMP, NUM_LEDS};

pub(super) fn calculate_led_data<const C: usize>(pixel_data: &mut [RGBA<u8>; C], temperature: f32) {
    let r_value = float_to_uint(temperature / HIGH_TEMP * 255.0 * 0.1);
    let b_value = float_to_uint((HIGH_TEMP - temperature) / HIGH_TEMP * 255.0 * 0.1);

    let num_leds = float_to_uint(temperature / HIGH_TEMP * (NUM_LEDS as f32 / 2.0)) as usize;
    for i in 0..num_leds {
        pixel_data[i] = RGBA::new(r_value as u8, 0, b_value as u8, 0x00);
    }
}

pub(super) fn calculate_humidity_data<const C: usize>(pixel_data : &mut [RGBA<u8>; C], humidity: f32) {
    if humidity <= 0.0 { return }

    let num_leds = NUM_LEDS - (float_to_uint(humidity / 100.0 * (NUM_LEDS as f32 / 2.0)) as usize);
    for i in num_leds..NUM_LEDS {
        pixel_data[i] = RGBA::new(0, 0x03, 0x05, 0);
    }
}
