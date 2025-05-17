use embedded_iconoir::{prelude::*, size16px::*};
use embedded_graphics::{
    image::Image,
    pixelcolor::BinaryColor,
    mono_font::{iso_8859_1::FONT_6X10, MonoTextStyleBuilder},
    text::{Baseline, Text},
    prelude::*,
    primitives::*
};
use heapless::String;
use super::data_structs::*;

pub(super) fn draw_icons(display: &mut Display) {
    let icn = weather::SunLight::new(BinaryColor::On);
    let img = Image::new(&icn, Point::zero());
    img.draw(display).unwrap();

    let icn = design_tools::Droplet::new(BinaryColor::On);
    let img = Image::new(&icn, Point::new(64, 0));
    img.draw(display).unwrap();

    let icn = activities::SeaWaves::new(BinaryColor::On);
    let img = Image::new(&icn, Point::new(0, 32));
    img.draw(display).unwrap();

    display.flush().unwrap();
}

pub(super) fn clear_old_data(display: &mut Display, location: &ItemLocation) {
    let rect_style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::Off)
        .build();

    Rectangle::new(location.rect_point(), location.rect_size())
        .into_styled(rect_style)
        .draw(display).unwrap();
}

pub(super) fn write_text(display: &mut Display, location: ItemLocation, text: String<12>) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    
    clear_old_data(display, &location);

    Text::with_baseline(&text, location.text_point(), text_style, Baseline::Top)
        .draw(display).unwrap();
}