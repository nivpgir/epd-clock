
#![feature(trait_alias)]

use std::sync::mpsc::Sender;
use chrono::Local;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use clocks::{AnalogClock};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait ThreadSafe = Send + Sync + 'static;
pub trait Waker<T: ThreadSafe> = FnOnce(Sender<T>) -> Result<()> + ThreadSafe + Copy;
pub trait ThreadsafeError = std::error::Error + ThreadSafe;


pub struct App<'a, S, D>
where
    // D: DrawTarget,
    // S: DrawDateTime<D> + MyScreen<D>,
{
    pub screen: &'a mut S,
    pub display: &'a mut D,
    // waker: &'static W
}

impl<'a, S, D> App<'a, S, D>
where
    // S: MyScreen<D>,
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
	let (sender, receiver) = std::sync::mpsc::channel();
	loop {
	    spawn_waker_thread(&sender, waker);
	    let frame_data = receiver.recv()?;
	    self.display.draw_current_date_time(frame_data)?;
	    // self.screen.my_update(&mut self.display);
	    self.screen.my_update(self.display);
	}

    }

    }
fn spawn_waker_thread<T: ThreadSafe>(sender: &Sender<T>, f: &'static impl Waker<T>) {
    let sender_clone = sender.clone();
    let _waker_thread =
        std::thread::Builder::new()
            .name("waker".into())
        .spawn(move || {f(sender_clone)});
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
    fn my_update(&mut self, display: &DT) -> ();
}

pub trait DrawDateTime{
    fn draw_current_date_time<FD: ThreadSafe>(&mut self, frame_data: FD)
					      -> Result<&mut Self>;
}

impl <D> DrawDateTime for D
where
    D: DrawTarget<Color=BinaryColor>,
    D::Error: ThreadsafeError{
    fn draw_current_date_time<FD: ThreadSafe>(&mut self, _frame_data: FD)
					      -> Result<&mut Self>{
	let time = Local::now();

        // MyText { time }.draw(&mut self.display)?;
	AnalogClock{ time }.draw(self)?;
	Ok(self)
    }
}


