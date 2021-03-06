
use chrono::{Local, Timelike};
use core::f32::consts::PI;
use embedded_graphics::{
    Drawable,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{
	Circle,
	Line,
	PrimitiveStyle,
	PrimitiveStyleBuilder,
	Rectangle,
	Sector
    }};

use embedded_graphics::geometry::AngleUnit;
/// The margin between the clock face and the display border.
const MARGIN: u32 = 10;

/// Converts a polar coordinate (angle/distance) into an (X, Y) coordinate centered around the
/// center of the circle.
///
/// The angle is relative to the 12 o'clock position and the radius is relative to the edge of the
/// clock face.
fn polar(circle: &Circle, angle: f32, radius_delta: i32) -> Point {
    let radius = circle.diameter as f32 / 2.0 + radius_delta as f32;

    circle.center()
        + Point::new(
            (angle.sin() * radius) as i32,
            -(angle.cos() * radius) as i32,
        )
}

/// Converts an hour into an angle in radians.
fn hour_to_angle(hour: u32) -> f32 {
    // Convert from 24 to 12 hour time.
    let hour = hour % 12;

    (hour as f32 / 12.0) * 2.0 * PI
}

/// Converts a sexagesimal (base 60) value into an angle in radians.
fn sexagesimal_to_angle(value: u32) -> f32 {
    (value as f32 / 60.0) * 2.0 * PI
}

/// Creates a centered circle for the clock face.
fn create_face(bounding_box: Rectangle) -> Circle {
    // The draw target bounding box can be used to determine the size of the display.
    // let bounding_box = target.bounding_box();
    let diameter = bounding_box.size.width.min(bounding_box.size.height) - 2 * MARGIN;

    Circle::with_center(bounding_box.center(), diameter)
}

// /// Draws a circle and 12 graduations as a simple clock face.

// /// Draw digital clock just above center with black text on a white background
// fn draw_digital_clock<D>(
//     target: &mut D,
//     clock_face: &Circle,
//     time_str: &str,
// ) -> Result<(), D::Error>
// where
//     D: DrawTarget<Color = BinaryColor>,
// {
//     // Create a styled text object for the time text.
//     let mut text = Text::new(
//         &time_str,
//         Point::zero(),
//         MonoTextStyle::new(&FONT_9X15, BinaryColor::Off),
//     );

//     // Move text to be centered between the 12 o'clock point and the center of the clock face.
//     text.translate_mut(
//         clock_face.center()
//             - text.bounding_box().center()
//             - clock_face.bounding_box().size.y_axis() / 4,
//     );

//     // Add a background around the time digits.
//     // Note that there is no bottom-right padding as this is added by the font renderer itself.
//     let text_dimensions = text.bounding_box();
//     Rectangle::new(
//         text_dimensions.top_left - Point::new(3, 3),
//         text_dimensions.size + Size::new(4, 4),
//     )
//     .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
//     .draw(target)?;

//     // Draw the text after the background is drawn.
//     text.draw(target)?;

//     Ok(())
// }

pub struct AnalogClock{
    pub time: chrono::DateTime<Local>
}

impl AnalogClock{
    fn draw_face<D>(&self, target: &mut D, clock_face: &Circle) -> Result<(), D::Error>
    where D: DrawTarget<Color = BinaryColor>, {
	// Draw the outer face.
	(*clock_face)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw(target)?;

	// Draw 12 graduations.
	for angle in (0..12).map(hour_to_angle) {
            // Start point on circumference.
            let start = polar(clock_face, angle, 0);

            // End point offset by 10 pixels from the edge.
            let end = polar(clock_face, angle, -10);

            Line::new(start, end)
		.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
		.draw(target)?;
	}

	Ok(())
    }

    /// Draws a clock hand.
    fn draw_hand<D>(&self, target: &mut D, clock_face: &Circle, angle: f32, length_delta: i32)
		    -> Result<(), D::Error>
    where D: DrawTarget<Color = BinaryColor> {
	let end = polar(clock_face, angle, length_delta);

	Line::new(clock_face.center(), end)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(target)
    }

    /// Draws a decorative circle on the second hand.
    fn draw_second_decoration<D>(
	&self,
	target: &mut D,
	clock_face: &Circle,
	angle: f32,
	length_delta: i32,
    ) -> Result<(), D::Error>
    where
	D: DrawTarget<Color = BinaryColor>,
    {
	let decoration_position = polar(clock_face, angle, length_delta);

	let decoration_style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::Off)
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();

	// Draw a fancy circle near the end of the second hand.
	Circle::with_center(decoration_position, 11)
            .into_styled(decoration_style)
            .draw(target)
    }
}

impl Drawable for AnalogClock{
    type Color = BinaryColor;

    type Output = ();

    fn draw<D>(&self, display: &mut D) -> std::result::Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color> {
	let clock_face = create_face(display.bounding_box());

	// Calculate the position of the three clock hands in radians.
        let hours_radians = hour_to_angle(self.time.hour());
        let minutes_radians = sexagesimal_to_angle(self.time.minute());
        let seconds_radians = sexagesimal_to_angle(self.time.second());

        // NOTE: In no-std environments, consider using
        // [arrayvec](https://stackoverflow.com/a/39491059/383609) and a fixed size buffer
        // let digital_clock_text = format!(
	//     "{:02}:{:02}:{:02}",
	//     time.hour(),
	//     time.minute(),
	//     time.second()
        // );

        display.clear(BinaryColor::Off)?;

        self.draw_face(display, &clock_face)?;
        self.draw_hand(display, &clock_face, hours_radians, -10)?;
        self.draw_hand(display, &clock_face, minutes_radians, -5)?;
        self.draw_hand(display, &clock_face, seconds_radians, 0)?;
        self.draw_second_decoration(display, &clock_face, seconds_radians, -20)?;

        // Draw digital clock just above center.
        // draw_digital_clock(display, &clock_face, &digital_clock_text)?;

        // Draw a small circle over the hands in the center of the clock face.
        // This has to happen after the hands are drawn so they're covered up.
        Circle::with_center(clock_face.center(), 9)
	    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
	    .draw(display)?;

	Ok(())
    }
}

pub struct MyClock{
    pub time: chrono::DateTime<Local>
}

impl Drawable for MyClock{
    type Color = BinaryColor;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color> {
	let face = create_face(target.bounding_box());

	let hours_angle = hour_to_angle(self.time.hour());
        let minutes_angle = sexagesimal_to_angle(self.time.minute());
        let seconds_angle = sexagesimal_to_angle(self.time.second());

	let diameter = face.diameter as f32;

	let hours_inner = 1.0 / 4.0 as f32;
	let hours_outer = 2.0 / 4.0 as f32;
	let minutes_inner = 3.0 / 5.0 as f32;
	let minutes_outer = 4.0 / 5.0 as f32;
	let seconds_inner = 5.0 / 6.0 as f32;
	let seconds_outer = 1.0 as f32;

 	let hours_inner =
	    Circle::with_center(target.bounding_box().center(),
				(diameter * hours_inner) as u32 + 2);
	let hours_outer =
	    Sector::with_center(target.bounding_box().center(),
				(diameter * hours_outer) as u32,
				90.0.deg(), -hours_angle.rad());
	let minutes_inner =
	    Circle::with_center(target.bounding_box().center(),
				(diameter * minutes_inner) as u32 + 2);
	let minutes_outer =
	    Sector::with_center(target.bounding_box().center(),
				(diameter * minutes_outer) as u32,
				90.0.deg(), -minutes_angle.rad());
	let seconds_inner =
	    Circle::with_center(target.bounding_box().center(),
				(diameter * seconds_inner) as u32 + 2);
	let seconds_outer =
	    Sector::with_center(target.bounding_box().center(),
				(diameter * seconds_outer) as u32,
				90.0.deg(), -seconds_angle.rad());


	seconds_outer.into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(target)?;
	seconds_inner.into_styled(PrimitiveStyle::with_fill(BinaryColor::Off)).draw(target)?;

	minutes_outer.into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(target)?;
	minutes_inner.into_styled(PrimitiveStyle::with_fill(BinaryColor::Off)).draw(target)?;

	hours_outer.into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(target)?;
	hours_inner.into_styled(PrimitiveStyle::with_fill(BinaryColor::Off)).draw(target)?;

	Ok(())
    }
}
