#![no_std]

pub mod colors;
pub mod container;
pub mod input;
pub mod painter;
pub mod style;
pub mod ui;
pub mod widgets;

pub use embedded_graphics::{mono_font::*, pixelcolor::*, prelude::*, primitives::*};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Capacity error: {0}")]
	Capacity(#[from] heapless::CapacityError),
	#[error("No space left inside layout")]
	NoSpaceLeft,
	#[error("Fatal internal error")]
	Fatal,
}
