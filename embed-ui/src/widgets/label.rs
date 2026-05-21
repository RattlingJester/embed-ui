use embedded_graphics::{
	mono_font::MonoTextStyle, prelude::DrawTarget, prelude::*, primitives::Rectangle, text::Text,
};

use heapless::String;

use crate::{
	input::Interaction,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[derive(Debug)]
pub struct Label {
	pub text:   String<MAX_TEXT_LEN>,
	pub bounds: Rectangle,
	pub focus:  bool,
}

impl Widget for Label {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,

		rect: &Rectangle,
		interaction: Option<Interaction>,

		target: &mut D,
	) -> Result<(), D::Error> {
		let char_style = MonoTextStyle::new(style.font, style.text_color);
		Text::new(&self.text, self.bounds.center(), char_style).draw(target)?;
		Ok(())
	}

	fn set_focus(&mut self, focus: bool) {
		todo!()
	}

	fn set_text(&mut self, text: &str) -> Result<(), crate::Error> {
		todo!()
	}

	fn size(&self) -> Size {
		todo!()
	}

	fn is_changed(&self) -> bool {
		todo!()
	}
}
