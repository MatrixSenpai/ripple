use embassy_rp::i2c::{Async, I2c};
use embassy_rp::peripherals::I2C0;

pub(super) const MPL_ADDR: u8 = 0x60;
pub(super) type MPLI2cType = I2c<'static, I2C0, Async>;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub(super) struct MPLCoefficients {
    pub(super) a0: f32,
    pub(super) b1: f32,
    pub(super) b2: f32,
    pub(super) c12: f32,
}