use embedded_graphics::prelude::DrawTarget;

use crate::{container::View, style::Style};

pub struct Ui<const S: usize, D: DrawTarget> {
	pub style: Style<D::Color>,
	error:     Option<D::Error>,
}

impl<const S: usize, D: DrawTarget> Ui<S, D> {
	pub const fn new(style: Style<D::Color>) -> Self {
		Self { style, error: None }
	}

	pub fn draw_view(&mut self, view: &View<'_, S>, target: &mut D) {
		for widget in view.iter() {
			match widget.draw(&self.style, target) {
				Ok(()) => (),
				Err(e) => {
					self.error = Some(e);
					panic!("Error occured: ");
				}
			}
		}
	}
}
