#![no_std]

pub mod alloc;
pub mod colors;
pub mod input;
pub mod page;
pub mod painter;
pub mod style;
pub mod ui;
pub mod widgets;

pub use embedded_graphics::{mono_font::*, pixelcolor::*, prelude::*, primitives::*};
pub use embedded_graphics_framebuf::FrameBuf;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Capacity error: {0}")]
	Capacity(#[from] heapless::CapacityError),
	#[error("No space left inside layout")]
	NoSpaceLeft,
	#[error(transparent)]
	Draw(#[from] core::convert::Infallible),
}
