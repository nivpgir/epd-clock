
#![feature(trait_alias)]

use std::sync::mpsc::Sender;
use chrono::Local;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::{Baseline, Text, TextStyleBuilder};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait ThreadSafe = Send + Sync + 'static;
pub trait Waker<T: ThreadSafe> = FnOnce(Sender<T>) -> Result<()> + ThreadSafe + Copy;
pub trait ThreadsafeError = std::error::Error + ThreadSafe;


pub struct App<'a, S, D> {
    pub screen: &'a mut S,
    pub display: &'a mut D,
}

impl<'a, S, D> App<'a, S, D>
where
    D: DrawTarget,
    S: MyScreen<D>,
{
    pub fn main_loop<T: ThreadSafe>(
	&mut self,
	waker: &'static impl Waker<T>,
    ) -> Result<()>
    where
	S: MyScreen<D>,
    	D: DrawTarget<Color=BinaryColor>,
	D::Error: ThreadsafeError {
	loop {
	    let (sender, receiver) = std::sync::mpsc::channel();
	    spawn_waker_thread(sender, waker);
	    let frame_data = receiver.recv()?;
	    let time = Local::now();
            self.display.clear(BinaryColor::Off)?;

            MyText { time }.draw(self.display)?;
	    // use clocks::AnalogClock;
	    // AnalogClock{ time }.draw(self)?;
	    use clocks::MyClock;
	    MyClock{ time }.draw(self.display)?;
	    // self.display.draw_current_date_time(&frame_data)?;
	    self.screen.my_update(self.display, &frame_data);
	}

    }

}
fn spawn_waker_thread<T: ThreadSafe>(sender: Sender<T>, f: &'static impl Waker<T>) {
    // let sender_clone = sender.clone();
    let _waker_thread =
        std::thread::Builder::new()
        .name("waker".into())
        .spawn(move || { f(sender)});
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait MyScreen<DT: DrawTarget>{
    fn my_update<UI: ThreadSafe>(&mut self, display: &DT, update_info: &UI) -> ();
}

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
        let character_style = MonoTextStyleBuilder::new()
            .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
            .background_color(Self::Color::Off)
            .text_color(Self::Color::On)
            .build();

        let start_pos = target.bounding_box().top_left;
        let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

        // self.display.clear_buffer(Color::White);
	let text = format!("{}", self.time.format("%Y\n%e %b\n%a"));
        Text::with_text_style(&text, start_pos, character_style, text_style).draw(target)?;

        Ok(())
    }
}
