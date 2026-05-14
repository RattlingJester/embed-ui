use embedded_graphics::{prelude::DrawTarget, primitives::Rectangle};

use crate::{style::Style, widgets::Widget};

#[derive(Debug)]
pub struct Checkbox<'a> {
	pub text:    &'a str,
	pub bounds:  Rectangle,
	pub focus:   bool,
	pub checked: bool,
}

impl<'a> Widget for Checkbox<'a> {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		target: &mut D,
	) -> Result<(), D::Error> {
		todo!()
	}
}
