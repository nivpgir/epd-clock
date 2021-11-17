
use epd_waveshare::{
    epd2in13_v2::{Display2in13, Epd2in13},
    prelude::*,
};
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    Delay, Spidev,
};
use rppal::gpio;

use chrono::{Local, Timelike};

use crate::Result;

pub struct MyEPDisplay {
    pub spi: Spidev,
    pub delay: Delay,
    // pub display: Display2in13,
    pub epd:
        Epd2in13<Spidev, gpio::OutputPin, gpio::InputPin, gpio::OutputPin, gpio::OutputPin, Delay>,
}

use app::{MyScreen, ThreadSafe};

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

    pub fn full_clear(self: &mut Self) -> Result<&mut Self> {
        self.epd
            .set_refresh(&mut self.spi, &mut self.delay, RefreshLut::Full)
            .and_then(|_| self.epd.clear_frame(&mut self.spi, &mut self.delay))
            .and_then(|_| self.epd.display_frame(&mut self.spi, &mut self.delay))?;
        Ok(self)
    }

    pub fn set_refresh(self: &mut Self, refresh_type: RefreshLut) -> Result<&mut Self> {
        self.epd
            .set_refresh(&mut self.spi, &mut self.delay, refresh_type)?;
        Ok(self)
    }
}

impl MyScreen<Display2in13> for MyEPDisplay{
    fn my_update<UI: ThreadSafe>(&mut self, display: &Display2in13, _update_info: &UI){
	let now = Local::now();
	let refresh_type = match (now.hour() % 12, now.minute(), now.second()) {
            (_, _, 0) => RefreshLut::Full,
            _ => RefreshLut::Quick,
	};
	self.set_refresh(refresh_type).unwrap();
	self.epd
	    .update_and_display_frame(&mut self.spi, display.buffer(), &mut self.delay)
	    .unwrap();
    }
}

