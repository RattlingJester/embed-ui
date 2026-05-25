use core::str::FromStr;

use embedded_graphics::{
	mono_font::MonoTextStyle, prelude::DrawTarget, prelude::*, primitives::Rectangle, text::Text,
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
	size:    Size,
	changed: bool,
}

impl Label {
	pub fn new(text: &str, size: Size) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			size,
			changed: true,
		})
	}

	pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text = String::from_str(text)?;
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
		let char_style = MonoTextStyle::new(style.font, style.text_color);
		Text::new(&self.text, rect.center(), char_style).draw(target)?;

		self.changed = false;

		Ok(())
	}

	fn interact(&mut self, _rect: &Rectangle, _interaction: Option<Interaction>) {}

	fn set_focus(&mut self, _focus: bool) {}

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
