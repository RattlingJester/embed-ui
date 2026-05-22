use embedded_graphics::{
	prelude::{DrawTarget, Size},
	primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
};

use crate::{input::Interaction, style::Style, widgets::Widget};

#[derive(Debug, Clone, PartialEq)]
pub struct Separator {
	size:    Size,
	changed: bool,
}

impl Separator {
	pub const fn new(size: Size) -> Self {
		Self {
			size,
			changed: false,
		}
	}
}

impl Widget for Separator {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		_interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		let style = PrimitiveStyleBuilder::new()
			.fill_color(style.border_color)
			.stroke_color(style.screen_bg)
			.stroke_width(style.border_width)
			.build();

		rect.draw_styled(&style, target)?;

		Ok(())
	}

	fn size(&self) -> Size {
		self.size
	}

	fn set_focus(&mut self, _focus: bool) {}

	fn is_changed(&self) -> bool {
		self.changed
	}

	fn is_focusable(&self) -> bool {
		false
	}
}
