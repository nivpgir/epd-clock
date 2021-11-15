
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
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
    // pub display: Display2in13,
    pub epd:
        Epd2in13<Spidev, gpio::OutputPin, gpio::InputPin, gpio::OutputPin, gpio::OutputPin, Delay>,
}

use app::MyScreen;

impl MyEPDisplay {
    pub fn new() -> Result<Self> {
        // Configure SPI
        let mut spi = Self::setup_spi()?;

        let (cs, busy, dc, rst) = Self::setup_gpios()?;

        let mut delay = Delay {};

        let epd = Epd2in13::new(&mut spi, cs, busy, dc, rst, &mut delay)?;

        // let display = Display2in13::default();
        return Ok(Self {
            spi,
            delay,
            // display: display,
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
    // pub fn set_rotation(self: &mut Self, rotation: DisplayRotation) -> &mut Self {
    //     self.display.set_rotation(rotation);
    //     self
    // }

    pub fn clear_screen(self: &mut Self) -> Result<&mut Self> {
        self.epd
            .set_refresh(&mut self.spi, &mut self.delay, RefreshLut::Full)
            .and_then(|_| self.epd.clear_frame(&mut self.spi, &mut self.delay))
            .and_then(|_| self.epd.display_frame(&mut self.spi, &mut self.delay))?;
        Ok(self)
    }
    // pub fn flush(&mut self) -> Result<&mut Self>{
    // 	self.epd.update_and_display_frame(&mut self.spi, self.display.buffer(), &mut self.delay)?;
    // 	Ok(self)

    // }

    // pub fn draw_current_date_time(self: &mut Self) -> Result<&mut Self> {
    //     let time = Local::now();
    //     // MyText { time }.draw(&mut self.display)?;
    // 	AnalogClock{ time }.draw(&mut self.display)?;
    //     self.epd
    //         .update_and_display_frame(&mut self.spi, self.display.buffer(), &mut self.delay)?;
    //     Ok(self)
    // }

    pub fn set_refresh(self: &mut Self, refresh_type: RefreshLut) -> Result<&mut Self> {
        self.epd
            .set_refresh(&mut self.spi, &mut self.delay, refresh_type)?;
        Ok(self)
    }
}

impl MyScreen<Display2in13> for MyEPDisplay{
    fn my_update(&mut self, display: &Display2in13){
	self.epd
	    .update_and_display_frame(&mut self.spi, display.buffer(), &mut self.delay)
	    .unwrap();
    }
}


// impl DrawTarget for MyEPDisplay{
//     type Color = BinaryColor;
//     type Error = Infallible;
//     fn draw_iter<I>(&mut self, pixels: I) -> std::result::Result<(), Self::Error>
//     where
//         I: IntoIterator<Item = Pixel<Self::Color>> {
// 	self.display.draw_iter(pixels)
//     }
// }

// impl OriginDimensions for MyEPDisplay{
//     fn size(&self) -> Size {
// 	self.display.size()
//     }
// }

struct MyText {
    time: chrono::DateTime<Local>,
}

impl Drawable for MyText {
    type Color = BinaryColor;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> std::result::Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let style = MonoTextStyleBuilder::new()
            .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
            .background_color(White)
            .text_color(Black)
            .build();

        let start_pos = target.bounding_box().top_left;
        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        // self.display.clear_buffer(Color::White);
	let text = format!("{}", self.time.format("%Y\n%a %e %b\n%T"));
        Text::with_text_style(&text, start_pos, style, text_style).draw(target)?;

        Ok(())
    }
}
