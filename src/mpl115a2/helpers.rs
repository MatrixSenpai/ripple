use super::data_structs::*;
use super::macros::*;

pub(super) async fn initialize_temp_i2c(handle: &mut MPLI2cType) -> MPLCoefficients {
    let mut coefficient_data: [u8; 8] = [0; 8];
    handle.write_read_async(MPL_ADDR, [A0_MSB], &mut coefficient_data).await.unwrap();

    debug!("Raw coeff data: {:#x}", coefficient_data);

    let a0_cf = ((coefficient_data[0] as i16) << 8) | (coefficient_data[1] as i16);
    let b1_cf = ((coefficient_data[2] as i16) << 8) | (coefficient_data[3] as i16);
    let b2_cf = ((coefficient_data[4] as i16) << 8) | (coefficient_data[5] as i16);
    let c12_cf = (((coefficient_data[6] as i16) << 8) | (coefficient_data[7] as i16)) >> 2;

    debug!("joined coeff data: {:#x} / {:#x} / {:#x} / {:#x}", a0_cf, b1_cf, b2_cf, c12_cf);

    let a0 = (a0_cf as f32) / 8.0;
    let b1 = (b1_cf as f32) / 8192.0;
    let b2 = (b2_cf as f32) / 16384.0;
    let c12 = (c12_cf as f32) / 4194304.0;

    MPLCoefficients { a0, b1, b2, c12 }
}

pub(super) async fn get_i2c_temp_data(handle: &mut MPLI2cType, coefficient_data: MPLCoefficients) -> (f32, f32) {
    let MPLCoefficients { a0, b1, b2, c12 } = coefficient_data;

    let mut temp_data: [u8; 4] = [0; 4];
    handle.write_read_async(MPL_ADDR, [PADC_MSB], &mut temp_data).await.unwrap();

    let pressure_raw = ((((temp_data[0] as u16) << 8) | temp_data[1] as u16) >> 6) as f32;
    let temperature_raw = ((((temp_data[2] as u16) << 8) | temp_data[3] as u16) >> 6) as f32;

    let pressure_computation = a0 + (b1 + c12 * temperature_raw) * pressure_raw + b2 * temperature_raw;

    let pressure = ((65.0 / 1023.0) * pressure_computation) + 50.0;
    let temperature = (temperature_raw - 498.0) / -5.35 + 25.0;

    (temperature, pressure)
}
