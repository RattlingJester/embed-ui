use embedded_graphics::{
	prelude::{DrawTarget, Size},
	primitives::Rectangle,
};

use heapless::String;

use crate::{
	input::Interaction,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Textbox {
	text:    String<MAX_TEXT_LEN>,
	size:    Size,
	changed: bool,
}

impl Widget for Textbox {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		todo!()
	}

	fn set_focus(&mut self, _focus: bool) {}

	fn size(&self) -> Size {
		self.size
	}

	fn is_focusable(&self) -> bool {
		false
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
