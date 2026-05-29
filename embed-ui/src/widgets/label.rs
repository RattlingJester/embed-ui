use core::str::FromStr;

use embedded_graphics::{
	mono_font::{MonoFont, MonoTextStyle},
	prelude::{DrawTarget, *},
	primitives::Rectangle,
	text::Text,
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

impl Widget for Label {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		target: &mut D,
	) -> Result<(), D::Error> {
		let char_style = MonoTextStyle::new(self.font, style.text_color);
		Text::new(&self.text, rect.center(), char_style).draw(target)?;

		Ok(())
	}

	fn interact(&mut self, _rect: &Rectangle, _interaction: Option<Interaction>) {}

	fn set_focus(&mut self, _focus: bool) {}

	fn mark_clean(&mut self) {
		self.changed = false;
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_focusable(&self) -> bool {
		false
	}

	fn is_dirty(&self) -> bool {
		self.changed
	}
}
