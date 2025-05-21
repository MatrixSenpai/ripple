use embassy_rp::gpio::Output;
use embassy_rp::peripherals::SPI0;
use embassy_rp::spi::{Blocking, Spi};
use embedded_graphics::geometry::{Point, Size};
use sh1106_driver::interface::SpiInterface;
use sh1106_driver::mode::GraphicsMode;

pub(super) type Display = GraphicsMode<SpiInterface<Spi<'static, SPI0, Blocking>, Output<'static>, Output<'static>>>;

pub(super) enum ItemLocation {
    Temperature,
    Pressure,
    Humidity,
}
impl ItemLocation {
    pub(super) fn rect_point(&self) -> Point {
        match self {
            Self::Temperature => Point::new(18, 0),
            Self::Pressure => Point::new(18, 32),
            Self::Humidity => Point::new(82, 0),
        }
    }
    pub(super) fn rect_size(&self) -> Size { Size::new(46, 14) }
    pub(super) fn text_point(&self) -> Point {
        match self {
            Self::Temperature => Point::new(19, 2),
            Self::Pressure => Point::new(19, 34),
            Self::Humidity => Point::new(83, 2),
        }
    }
}

