use core::str::FromStr;

use embedded_graphics::{
	mono_font::{MonoFont, MonoTextStyle},
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
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
pub struct Button {
	text:      String<MAX_TEXT_LEN>,
	font:      &'static MonoFont<'static>,
	size:      Size,
	focus:     bool,
	focusable: bool,
	changed:   bool,
	pressed:   bool,
}

impl Button {
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
		})
	}

	pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}

	pub fn is_clicked(&self) -> bool {
		self.pressed
	}
}

impl Widget for Button {
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

	fn mark_clean(&mut self) {
		self.changed = false;
	}

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus && self.focusable {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn set_focusable(&mut self, focusable: bool) {
		self.focusable = focusable
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_pressed(&self) -> bool {
		self.pressed
	}

	fn is_focusable(&self) -> bool {
		true
	}

	fn is_dirty(&self) -> bool {
		self.changed
	}
}
