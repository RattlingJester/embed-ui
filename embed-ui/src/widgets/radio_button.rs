use core::str::FromStr;

use embedded_graphics::{
	Drawable,
	mono_font::{MonoFont, MonoTextStyle},
	prelude::{PixelColor, Point, Primitive, Size},
	primitives::{Line, PrimitiveStyleBuilder, Rectangle},
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_framebuf::FrameBuf;
use heapless::String;

use crate::{
	Error,
	input::Interaction,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub struct RadioButton {
	text:      String<MAX_TEXT_LEN>,
	font:      &'static MonoFont<'static>,
	size:      Size,
	focus:     bool,
	focusable: bool,
	changed:   bool,
	pressed:   bool,
	toggled:   bool,
}

impl RadioButton {
	pub fn new(
		text: &str,
		font: &'static MonoFont<'static>,
		size: Size,
		focusable: bool,
	) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			font,
			size,
			focus: false,
			focusable,
			changed: true,
			pressed: false,
			toggled: false,
		})
	}
}
impl<C: PixelColor, const F: usize> Widget<C, F> for RadioButton {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; F]>,
	) -> Result<(), Error> {
		let bg = if self.pressed {
			style.active_color
		} else {
			style.bg_color
		};

		let border_color = match self.focus {
			true => style.focus_color,
			false => style.border_color,
		};

		let prim_style = PrimitiveStyleBuilder::new()
			.stroke_color(border_color)
			.stroke_width(style.border_width)
			.fill_color(bg)
			.build();

		rect.into_styled(prim_style).draw(target)?;

		if self.toggled {
			let line_thickness = 5;
			let line_style = PrimitiveStyleBuilder::new()
				.stroke_color(style.focus_color)
				.stroke_width(line_thickness)
				.build();

			let start_x = rect.top_left.x + (rect.size.width as i32 / 4);
			let end_x = rect.top_left.x + ((rect.size.width as i32 * 3) / 4) - 1;
			let y = rect.top_left.y + rect.size.height as i32 - (line_thickness as i32 / 2) - 1;

			Line::new(Point::new(start_x, y), Point::new(end_x, y))
				.into_styled(line_style)
				.draw(target)?;
		}

		let ts = TextStyleBuilder::new()
			.alignment(Alignment::Center)
			.baseline(Baseline::Middle)
			.build();

		let text_location = Point::new(
			rect.top_left.x + (rect.size.width / 2) as i32,
			rect.top_left.y + (rect.size.height / 2) as i32,
		);

		Text::with_text_style(
			&self.text,
			text_location,
			MonoTextStyle::new(self.font, style.text_color),
			ts,
		)
		.draw(target)?;

		Ok(())
	}

	fn interact(&mut self, interaction: Option<Interaction>) {
		match interaction {
			Some(Interaction::Click(_)) if !self.pressed => {
				self.pressed = true;
				self.changed = true;
			}
			Some(Interaction::Release(_)) if self.pressed => {
				self.pressed = false;
				self.changed = true;
			}
			None if self.pressed => {
				self.pressed = false;
				self.changed = true;
			}
			_ => (),
		}
	}

	fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}

	fn set_active(&mut self, active: bool) {
		if self.toggled != active {
			self.toggled = active;
			self.changed = true;
		}
	}

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus && self.focusable {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn set_focusable(&mut self, focusable: bool) {
		self.focusable = focusable;
	}

	fn mark_clean(&mut self) {
		self.changed = false;
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_focusable(&self) -> bool {
		self.focusable
	}

	fn is_changed(&self) -> bool {
		self.changed
	}

	fn is_pressed(&self) -> bool {
		self.pressed
	}
}
