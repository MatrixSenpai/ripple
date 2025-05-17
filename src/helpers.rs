
pub fn rounded_for_display(n: f32) -> f32 {
    use embassy_rp::rom_data::float_funcs::*;
    int64_to_float(float_to_int64(n * 100.0)) / 100.0
}

pub fn rotate_calculate_average<const C: usize>(buffer: &mut [f32; C], new_item: Option<f32>) -> f32 {
    if let Some(val) = new_item {
        for i in (0..C - 2).rev() {
            buffer[i + 1] = buffer[i];
        }
        buffer[0] = val;
    }

    let buffer_slice = buffer.iter().filter(|v| **v != 0.0).count() as f32;
    buffer.iter().fold(0.0, |l, r| l + r) / buffer_slice
}

