#![no_std]

pub mod container;
pub mod input;
pub mod style;
pub mod ui;
pub mod widgets;

pub use embedded_graphics::{prelude::*, primitives::Rectangle};

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
