use embedded_graphics::{
	prelude::{PixelColor, Size},
	primitives::Rectangle,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{Error, input::Interaction, style::Style};

pub mod button;
pub mod checkbox;
pub mod label;
pub mod radio_button;
pub mod separator;
pub mod textbox;

pub const MAX_TEXT_LEN: usize = 32;

pub trait Widget<C: PixelColor, const FB_SIZE: usize> {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; FB_SIZE]>,
	) -> Result<(), Error>;

	fn interact(&mut self, _interaction: Option<Interaction>) -> bool {
		false
	}

	fn set_focus(&mut self, _focus: bool) {}
	fn set_focusable(&mut self, _focusable: bool) {}
	fn mark_clean(&mut self);

	fn size(&self) -> Size;
	fn is_focusable(&self) -> bool {
		false
	}
	fn is_dirty(&self) -> bool;
}
