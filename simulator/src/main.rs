use std::sync::mpsc::Sender;

use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::geometry::Size;
use embedded_graphics::Pixel;
use embedded_graphics::prelude::OriginDimensions;
use embedded_graphics::prelude::DrawTarget;

use app::{
    App,
    Result,
    MyScreen
};

fn main() -> Result<()> {
    let simulator = SimulatorDisplay::<BinaryColor>::new(Size::new(256, 256));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut screen = Window::new("Clock", &output_settings);
    let mut my_simulator = MySimulator{simulator};
    App{screen: &mut screen, display: &mut my_simulator}.main_loop(&waker)
}

fn waker<>(sender: Sender<()>) -> Result<()> {
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sender.send(())?;
    Ok(())
}


pub struct MySimulator{
    simulator: SimulatorDisplay<BinaryColor>
}

impl DrawTarget for MySimulator{
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> std::result::Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
	self.simulator.draw_iter(pixels)
    }
}

impl OriginDimensions for MySimulator{
    fn size(&self) -> Size {
        self.simulator.size()
    }

}

impl MyScreen<MySimulator> for Window{
    fn my_update(&mut self, display: &MySimulator){
	self.update(&display.simulator)
    }
}
