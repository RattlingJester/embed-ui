use core::str::FromStr;

use embedded_graphics::{
	Drawable,
	mono_font::{MonoFont, MonoTextStyle},
	prelude::{DrawTarget, Point, Primitive, Size},
	primitives::{Line, PrimitiveStyleBuilder, Rectangle},
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use heapless::String;

use crate::{
	Error,
	input::Interaction,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
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

	pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}

	pub fn set_toggle(&mut self, toggle: bool) {
		if self.toggled != toggle {
			self.toggled = toggle;
			self.changed = true;
		}
	}

	pub fn is_clicked(&self) -> bool {
		self.pressed
	}
}
impl Widget for RadioButton {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		target: &mut D,
	) -> Result<(), D::Error> {
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

	fn interact(&mut self, rect: &Rectangle, interaction: Option<Interaction>) {
		let new_pressed = matches!(
			interaction,
			Some(Interaction::Click(p)) if rect.contains(p)
		);
		let released = matches!(
			interaction,
			Some(Interaction::Release(p)) if rect.contains(p)
		);

		if released && self.pressed {
			self.pressed = true;
			self.changed = true;
		}

		if new_pressed != self.pressed {
			self.pressed = new_pressed;
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

	fn is_pressed(&self) -> bool {
		self.pressed
	}

	fn is_focusable(&self) -> bool {
		self.focusable
	}

	fn is_dirty(&self) -> bool {
		self.changed
	}
}
