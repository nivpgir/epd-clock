
use std::sync::mpsc::Sender;

use rpi_epd2in13;
use chrono::{Duration, Local, Timelike};
use epd_waveshare::{
    epd2in13_v2::Display2in13,
    graphics::{Display, DisplayRotation},
    prelude::RefreshLut
};
use rpi_epd2in13::MyEPDisplay;
use app::App;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let mut epd2in13 = MyEPDisplay::new()?;
    let mut display = Display2in13::default();
    display.set_rotation(DisplayRotation::Rotate180);

    epd2in13.clear_screen()?;

    App{ screen: &mut epd2in13, display: &mut display }
    .main_loop(&thread_waker)?;
    // App{epd2in13.epd, epd2in13.display}.main_loop();
    // let (sender, receiver) = sync::mpsc::channel();
    // loop {
    //     spawn_waker_thread(&sender);
    //     let refresh_type = receiver.recv()?;
    //     epd2in13
    //         .set_refresh(refresh_type)
    // 	    ?.draw_current_date_time()
    // 	    ?.flush()?;
    // }
    Ok(())
}


// fn spawn_waker_thread(sender: &Sender<RefreshLut>) {
//     let sender_clone = sender.clone();
//     let _waker_thread =
//         std::thread::Builder::new()
//             .name("waker".into())
//             .spawn(move || -> Result<()> {
//                 let now = Local::now();
//                 let cur_ms = now.timestamp_subsec_millis().into();
//                 let time_to_next_wake = Duration::seconds(1) - Duration::milliseconds(cur_ms);

//                 std::thread::sleep(time_to_next_wake.to_std()?);
//                 match (now.hour() % 12, now.minute(), now.second()) {
//                     (0, 0, 0) => sender_clone.send(RefreshLut::Full),
//                     _ => sender_clone.send(RefreshLut::Quick),
//                 }?;
//                 Ok(())
//             });
// }


fn thread_waker(sender: Sender<RefreshLut>) -> Result<()> {
    let now = Local::now();
    let cur_ms = now.timestamp_subsec_millis().into();
    let time_to_next_wake = Duration::seconds(1) - Duration::milliseconds(cur_ms);

    std::thread::sleep(time_to_next_wake.to_std()?);
    match (now.hour() % 12, now.minute(), now.second()) {
        (0, 0, 0) => sender.send(RefreshLut::Full),
        _ => sender.send(RefreshLut::Quick),
    }?;
    Ok(())
}
