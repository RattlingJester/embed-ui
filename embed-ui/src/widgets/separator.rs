use embedded_graphics::{
	prelude::{PixelColor, Size},
	primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{Error, style::Style, widgets::Widget};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
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

impl<C: PixelColor, const F: usize> Widget<C, F> for Separator {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; F]>,
	) -> Result<(), Error> {
		let style = PrimitiveStyleBuilder::new()
			.fill_color(style.border_color)
			.stroke_color(style.screen_bg)
			.stroke_width(style.border_width)
			.build();

		rect.draw_styled(&style, target)?;

		Ok(())
	}

	fn mark_clean(&mut self) {
		self.changed = false
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
