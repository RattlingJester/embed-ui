use core::str::FromStr;

use embedded_graphics::{
	Drawable,
	mono_font::{MonoFont, MonoTextStyle},
	prelude::{DrawTarget, Point, Size},
	primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};

use heapless::String;

use crate::{
	Error,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub struct Textbox {
	text:      String<MAX_TEXT_LEN>,
	font:      &'static MonoFont<'static>,
	size:      Size,
	focus:     bool,
	focusable: bool,
	changed:   bool,
}

impl Textbox {
	pub fn new(text: &str, font: &'static MonoFont, size: Size) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			font,
			size,
			focus: false,
			focusable: true,
			changed: true,
		})
	}

	pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}
}

impl Widget for Textbox {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		target: &mut D,
	) -> Result<(), D::Error> {
		let border_color = match self.focus {
			true => style.focus_color,
			false => style.border_color,
		};

		let border_style = PrimitiveStyleBuilder::new()
			.stroke_color(border_color)
			.stroke_width(style.border_width)
			.fill_color(style.bg_color)
			.build();

		let padding_x = 4;

		let ts = TextStyleBuilder::new()
			.alignment(Alignment::Left)
			.baseline(Baseline::Middle)
			.build();

		let text_pos = Point::new(
			rect.top_left.x + padding_x,
			rect.top_left.y + rect.size.height as i32 / 2,
		);

		rect.draw_styled(&border_style, target)?;

		Text::with_text_style(
			&self.text,
			text_pos,
			MonoTextStyle::new(self.font, style.text_color),
			ts,
		)
		.draw(target)?;

		Ok(())
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

	fn mark_clean(&mut self) {
		self.changed = false
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_dirty(&self) -> bool {
		self.changed
	}
}
