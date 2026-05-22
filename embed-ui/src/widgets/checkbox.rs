use embedded_graphics::{
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
};

use crate::{input::Interaction, style::Style, widgets::Widget};

#[derive(Debug, Clone, PartialEq)]
pub struct Checkbox {
	size:    Size,
	focus:   bool,
	checked: bool,
	changed: bool,
}

impl Checkbox {
	pub fn new(size: Size) -> Self {
		Self {
			size,
			focus: false,
			checked: false,
			changed: true,
		}
	}
}

impl Widget for Checkbox {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		let bg = match interaction {
			None => style.bg_color,
			_ => todo!(),
		};

		let border_color = match self.focus {
			true => style.focus_color,
			false => style.border_color,
		};

		let border_style = PrimitiveStyleBuilder::new()
			.stroke_color(border_color)
			.stroke_width(style.border_width)
			.fill_color(bg)
			.build();

		if self.checked {
			let shortest = rect.size.width.min(rect.size.height);
			let pad = (shortest / 4).max(2) as i32;
			let inner = Rectangle::new(
				rect.top_left + Point::new(pad, pad),
				Size::new(
					rect.size.width.saturating_sub(pad as u32 * 2),
					rect.size.height.saturating_sub(pad as u32 * 2),
				),
			);

			let fill_style = PrimitiveStyleBuilder::new()
				.fill_color(style.active_color)
				.build();

			inner.into_styled(fill_style).draw(target)?;
		}

		rect.into_styled(border_style).draw(target)?;

		self.changed = false;

		Ok(())
	}

	fn size(&self) -> Size {
		self.size
	}

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn is_focusable(&self) -> bool {
		true
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
