use embedded_graphics::{
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{Error, input::Interaction, style::Style, widgets::Widget};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub struct Checkbox {
	size:      Size,
	focus:     bool,
	focusable: bool,
	checked:   bool,
	held:      bool,
	changed:   bool,
}

impl Checkbox {
	pub fn new(size: Size, focusable: bool) -> Self {
		Self {
			size,
			focus: false,
			focusable,
			checked: false,
			held: false,
			changed: true,
		}
	}
}

impl<C: PixelColor, const F: usize> Widget<C, F> for Checkbox {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; F]>,
	) -> Result<(), Error> {
		let bg = if self.checked {
			style.active_color
		} else {
			style.bg_color
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

		Ok(())
	}

	fn interact(&mut self, interaction: Option<Interaction>) -> bool {
		let new_pressed = matches!(interaction, Some(Interaction::Click(_)));

		if new_pressed && !self.held {
			self.checked = !self.checked;
			self.held = true;
			self.changed = true;
			return true;
		}

		if !new_pressed {
			self.held = false;
			return true;
		}

		false
	}

	fn mark_clean(&mut self) {
		self.changed = false;
	}

	fn size(&self) -> Size {
		self.size
	}

	fn set_focusable(&mut self, focusable: bool) {
		self.focusable = focusable;
	}

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus && self.focusable {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn is_focusable(&self) -> bool {
		self.focusable
	}

	fn is_dirty(&self) -> bool {
		self.changed
	}
}
