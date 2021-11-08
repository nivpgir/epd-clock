
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};

use epd_waveshare::{
    color::*,
    epd2in13_v2::{Display2in13, Epd2in13},
    prelude::*,
};
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    Delay, Spidev,
};
use rppal::gpio;

use chrono::Local;

use crate::Result;

pub struct MyEPDisplay {
    pub spi: Spidev,
    pub delay: Delay,
    pub display: Display2in13,
    pub epd: Epd2in13
	<Spidev, gpio::OutputPin, gpio::InputPin, gpio::OutputPin, gpio::OutputPin, Delay>,
}

impl MyEPDisplay {
    pub fn new() -> Result<Self> {
        // Configure SPI
        let mut spi = Self::setup_spi()?;

        let (cs, busy, dc, rst) = Self::setup_gpios()?;

        let mut delay = Delay {};

        let epd = Epd2in13::new(&mut spi, cs, busy, dc, rst, &mut delay)?;

        return Ok(Self {
            spi,
            delay,
            display: Display2in13::default(),
            epd,
        });
    }
    fn setup_spi() -> Result<Spidev> {
        let mut spi = Spidev::open("/dev/spidev0.0")?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(4_000_000)
            .mode(spidev::SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;
        return Ok(spi);
    }

    fn setup_gpios() -> Result<(
        gpio::OutputPin,
        gpio::InputPin,
        gpio::OutputPin,
        gpio::OutputPin,
    )> {
        let gpio = gpio::Gpio::new()?;
        let mut cs = gpio.get(8)?.into_output(); //PIN 24 CE0
        cs.set_high();
        let busy = gpio.get(24)?.into_input(); //pin 18
        let mut dc = gpio.get(25)?.into_output(); //pin 22 //bcm6
        dc.set_high();
        let mut rst = gpio.get(17)?.into_output(); //pin 11 //bcm16
        rst.set_high();
        return Ok((cs, busy, dc, rst));
    }
    pub fn set_rotation(self: &mut Self, rotation: DisplayRotation) -> &mut Self{
	self.display.set_rotation(rotation);
	self
    }
    
    pub fn clear_screen(self: &mut Self) -> Result<&mut Self> {
        self.epd
            .set_refresh(&mut self.spi, &mut self.delay, RefreshLut::Full)
            .and_then(|_| self.epd.clear_frame(&mut self.spi, &mut self.delay))
            .and_then(|_| self.epd.display_frame(&mut self.spi, &mut self.delay))?;
        Ok(self)
    }

    pub fn draw_current_date_time(self: &mut Self) -> Result<&Self> {
        let time_string = format!("{}", Local::now().format("%Y\n%a %e %b\n%T"));
        self.draw_text(time_string.as_str(), 0, 40);
        self.epd
            .update_and_display_frame(&mut self.spi, self.display.buffer(), &mut self.delay)?;
        Ok(self)
    }

    fn draw_text(self: &mut Self, text: &str, x: i32, y: i32) {
        let style = MonoTextStyleBuilder::new()
            // .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
            .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
            .text_color(White)
            .background_color(Black)
            .build();

        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        let _ = Text::with_text_style(text, Point::new(x, y), style, text_style)
            .draw(&mut self.display);
    }

    pub fn set_refresh(self: &mut Self, refresh_type: RefreshLut) -> Result<&mut Self> {
	self.epd.set_refresh(&mut self.spi, &mut self.delay, refresh_type)?;
	Ok(self)
    }
}


