#![allow(unused)]

macro_rules! mplregs {
    ($($name:ident : $val:expr),* $(,)?) => {
        $(
            pub const $name: u8 = $val;
        )*

        pub fn regname(reg: u8) -> &'static str {
            match reg {
                $(
                    $val => stringify!($name),
                )*
                _ => panic!("bad reg"),
            }
        }
    }
}

mplregs! {
    PADC_MSB: 0x00, // 10b pressure adc output MSB
    PADC_LSB: 0x01, // 10b pressure adc output LSB
    TADC_MSB: 0x02, // 10b temperature adc output MSB
    TADC_LSB: 0x03, // 10b temperature adc output LSB
    A0_MSB: 0x04,   // A0 coefficient MSB
    A0_LSB: 0x05,   // A0 coefficient LSB
    B1_MSB: 0x06,   // B1 coefficient MSB
    B1_LSB: 0x07,   // B1 coefficient LSB
    B2_MSB: 0x08,   // B2 coefficient MSB
    B2_LSB: 0x09,   // B2 coefficient LSB
    C12_MSB: 0x0A,  // C12 coefficient MSB
    C12_LSB: 0x0B,  // C12 coefficient LSB
    CONVERT: 0x12,  // Start pressure/temperature conversion
}