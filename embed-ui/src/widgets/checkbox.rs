use embedded_graphics::{
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
};

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
		let border_style = PrimitiveStyleBuilder::new()
			.stroke_color(style.border_color)
			.stroke_width(style.border_width)
			.fill_color(style.bg_color)
			.build();

		self.bounds.into_styled(border_style).draw(target)?;

		if self.checked {
			let shortest = self.bounds.size.width.min(self.bounds.size.height);
			let pad = (shortest / 4).max(2) as i32;
			let inner = Rectangle::new(
				self.bounds.top_left + Point::new(pad, pad),
				Size::new(
					self.bounds.size.width.saturating_sub(pad as u32 * 2),
					self.bounds.size.height.saturating_sub(pad as u32 * 2),
				),
			);

			let fill_style = PrimitiveStyleBuilder::new()
				.fill_color(style.active_color)
				.build();

			inner.into_styled(fill_style).draw(target)?;
		}

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
