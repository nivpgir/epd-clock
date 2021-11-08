use std::sync::mpsc::Sender;

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

use chrono::{Duration, Local, Timelike};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let mut epd2in13 = MyEPDScreen::new()?;
    let mut delay = Delay {};

    epd2in13.display.set_rotation(DisplayRotation::Rotate90);
    epd2in13.clear_screen()?;
    let (sender, receiver) = std::sync::mpsc::channel();
    loop {
        spawn_waker_thread(&sender);
        let refresh_type = receiver.recv()?;
        epd2in13
            .epd
            .set_refresh(&mut epd2in13.spi, &mut delay, refresh_type)?;

        epd2in13.draw_current_date_time()?;
    }
}

struct MyEPDScreen {
    pub spi: Spidev,
    pub delay: Delay,
    pub display: Display2in13,
    pub epd:
        Epd2in13<Spidev, gpio::OutputPin, gpio::InputPin, gpio::OutputPin, gpio::OutputPin, Delay>,
}

impl MyEPDScreen {
    fn new() -> Result<Self> {
        // Configure SPI
        // Settings are taken from
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
    fn clear_screen(self: &mut Self) -> Result<&mut Self> {
        self.epd
            .set_refresh(&mut self.spi, &mut self.delay, RefreshLut::Full)
            .and_then(|_| self.epd.clear_frame(&mut self.spi, &mut self.delay))
            .and_then(|_| self.epd.display_frame(&mut self.spi, &mut self.delay))?;
        Ok(self)
    }

    fn draw_current_date_time(self: &mut Self) -> Result<&Self> {
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
}

fn spawn_waker_thread(sender: &Sender<RefreshLut>) {
    let sender_clone = sender.clone();
    let _waker_thread = std::thread::Builder::new()
        .name("waker".into())
        .spawn(move || -> Result<()> {
            let now = Local::now();
            let cur_ms = now.timestamp_subsec_millis().into();
            let time_to_next_wake = Duration::seconds(1) - Duration::milliseconds(cur_ms);

            std::thread::sleep(time_to_next_wake.to_std()?);
            match (now.hour() % 12, now.minute(), now.second()) {
                (6, 0, 0) | (0, 0, 0) => sender_clone.send(RefreshLut::Full),
                _ => sender_clone.send(RefreshLut::Quick),
            }?;
	    Ok(())
        });
}
