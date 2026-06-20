use core::str::FromStr;

use embedded_graphics::{
	Drawable,
	mono_font::{MonoFont, MonoTextStyle},
	prelude::{PixelColor, Point, Size},
	primitives::Rectangle,
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};

use embedded_graphics_framebuf::FrameBuf;
use heapless::String;

use crate::{
	Error,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
	text:    String<MAX_TEXT_LEN>,
	font:    &'static MonoFont<'static>,
	size:    Size,
	changed: bool,
}

impl Label {
	pub fn new(text: &str, font: &'static MonoFont, size: Size) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			font,
			size,
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

impl<C: PixelColor, const F: usize> Widget<C, F> for Label {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; F]>,
	) -> Result<(), Error> {
		let char_style = MonoTextStyle::new(self.font, style.text_color);
		let text_style = TextStyleBuilder::new()
			.baseline(Baseline::Middle)
			.alignment(Alignment::Center)
			.build();

		let center_point = Point::new(
			rect.top_left.x + (rect.size.width / 2) as i32,
			rect.top_left.y + (rect.size.height / 2) as i32,
		);

		Text::with_text_style(&self.text, center_point, char_style, text_style).draw(target)?;
		Ok(())
	}

	fn mark_clean(&mut self) {
		self.changed = false;
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_dirty(&self) -> bool {
		self.changed
	}

	fn set_focus(&mut self, _focus: bool) {}

	fn set_focusable(&mut self, _focusable: bool) {}

	fn is_focusable(&self) -> bool {
		false
	}
}
