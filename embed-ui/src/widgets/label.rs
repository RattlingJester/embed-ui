use embedded_graphics::{
	mono_font::MonoTextStyle, prelude::DrawTarget, prelude::*, primitives::Rectangle, text::Text,
};

use crate::{style::Style, widgets::Widget};

#[derive(Debug)]
pub struct Label<'a> {
	pub text:   &'a str,
	pub bounds: Rectangle,
	pub focus:  bool,
}

impl<'a> Widget for Label<'a> {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		target: &mut D,
	) -> Result<(), D::Error> {
		let char_style = MonoTextStyle::new(style.font, style.text_color);
		Text::new(self.text, self.bounds.center(), char_style).draw(target)?;
		Ok(())
	}

	fn update(&mut self) {
		todo!()
	}

	fn set_focus(&mut self, focus: bool) {
		todo!()
	}

	fn is_changed(&self) -> bool {
		todo!()
	}
}
