use embedded_graphics::prelude::DrawTarget;

use crate::{container::View, style::Style, widgets::Widget};

pub struct Ui<const S: usize, D: DrawTarget> {
	pub style: Style<D::Color>,
}

impl<const S: usize, D: DrawTarget> Ui<S, D> {
	pub const fn new(style: Style<D::Color>) -> Self {
		Self { style }
	}

	pub fn draw_view(&mut self, view: &mut View<'_, S>, target: &mut D) -> Result<(), D::Error> {
		for widget in view.iter_mut() {
			if widget.is_changed() {
				widget.draw(&self.style, target)?;
			}
		}

		Ok(())
	}
}
