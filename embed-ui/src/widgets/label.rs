use embedded_graphics::{prelude::DrawTarget, primitives::Rectangle};

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
		todo!()
	}
}
