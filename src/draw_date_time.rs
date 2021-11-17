
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::{BinaryColor, PixelColor};
use epd_waveshare::graphics::Display;
use epd_waveshare::prelude::Color;
use linux_embedded_hal::gpio_cdev::Error;
use crate::Result;
use crate::analog_clock::AnalogClock;
use chrono::Local;

pub trait DrawDateTime{
    fn draw_current_date_time(self: &mut Self) -> Result<&mut Self>;
}

pub trait GetDisplay{
    fn get_display(self: &mut Self);
}

impl<'a, T> DrawDateTime for T
where
    T::Error: 'static + std::error::Error + Send + Sync,
    T: DrawTarget<Color=BinaryColor>, {
    fn draw_current_date_time(self: &mut Self) -> Result<&mut Self>{
	let time = Local::now();
	AnalogClock{ time }.draw(self)?;
	Ok(self)
    }
}
