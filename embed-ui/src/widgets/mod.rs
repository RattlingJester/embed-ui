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

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Default, Clone, Copy)]
pub enum Horizontal {
	Left,
	#[default]
	Center,
	Right,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Default, Clone, Copy)]
pub enum Vertical {
	Top,
	#[default]
	Center,
	Bottom,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Default, Clone, Copy)]
pub struct TextAlignment {
	pub horizontal: Horizontal,
	pub vertical:   Vertical,
}

pub trait Widget<C: PixelColor, const FB_SIZE: usize> {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; FB_SIZE]>,
	) -> Result<(), Error>;

	fn interact(&mut self, _interaction: Option<Interaction>) {}

	fn set_active(&mut self, _active: bool) {}
	fn set_text(&mut self, _text: &str) -> Result<(), Error> {
		Ok(())
	}
	fn set_focus(&mut self, _focus: bool) {}
	fn set_focusable(&mut self, _focusable: bool) {}
	fn mark_clean(&mut self);

	fn size(&self) -> Size;
	fn is_changed(&self) -> bool;
	fn is_focusable(&self) -> bool {
		false
	}
	fn is_pressed(&self) -> bool {
		false
	}
}
