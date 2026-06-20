use core::str::FromStr;

use embedded_graphics::{
	mono_font::{MonoFont, MonoTextStyle},
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
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

impl<C: PixelColor, const F: usize> Widget<C, F> for Button {
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

	fn interact(&mut self, interaction: Option<Interaction>) -> bool {
		let new_pressed = matches!(interaction, Some(Interaction::Click(_)));
		let released = matches!(interaction, Some(Interaction::Release(_)));

		if released && self.pressed {
			self.pressed = false;
			self.changed = true;
		}

		if new_pressed != self.pressed {
			self.pressed = new_pressed;
			self.changed = true;

			true
		} else {
			false
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

	fn is_focusable(&self) -> bool {
		self.focusable
	}

	fn is_dirty(&self) -> bool {
		self.changed
	}
}
