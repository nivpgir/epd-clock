
use std::sync::mpsc::Sender;

mod my_ep_display;
use my_ep_display::MyEPDisplay;

use chrono::{Duration, Local};
use epd_waveshare::{
    epd2in13_v2::Display2in13,
    graphics::{Display, DisplayRotation},
};
use app::App;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let mut epd2in13 = MyEPDisplay::new()?;
    let mut display = Display2in13::default();
    display.set_rotation(DisplayRotation::Rotate180);

    epd2in13.full_clear()?;

    App{ screen: &mut epd2in13, display: &mut display }
    .main_loop(&thread_waker)?;
    Ok(())
}

fn thread_waker(sender: Sender<()>) -> Result<()> {
    let now = Local::now();
    let cur_ms = now.timestamp_subsec_millis().into();
    let time_to_next_wake = Duration::seconds(1) - Duration::milliseconds(cur_ms);

    std::thread::sleep(time_to_next_wake.to_std()?);
    sender.send(())?;
    Ok(())
}
