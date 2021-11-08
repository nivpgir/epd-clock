use std::sync::mpsc::Sender;

mod my_ep_display;
use chrono::{Duration, Local, Timelike};
use epd_waveshare::{graphics::DisplayRotation, prelude::RefreshLut};
use my_ep_display::MyEPDisplay;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let mut epd2in13 = MyEPDisplay::new()?;

    epd2in13
	.set_rotation(DisplayRotation::Rotate90)
        .clear_screen()?;
    let (sender, receiver) = std::sync::mpsc::channel();
    loop {
        spawn_waker_thread(&sender);
        let refresh_type = receiver.recv()?;
        epd2in13.set_refresh(refresh_type)
	    ?.draw_current_date_time()?;

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
