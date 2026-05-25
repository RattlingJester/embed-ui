#![no_std]

pub mod container;
pub mod input;
pub mod style;
pub mod ui;
pub mod widgets;

pub use embedded_graphics::{pixelcolor::*, prelude::*, primitives::Rectangle};

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

fn rects_overlap(a: &Rectangle, b: &Rectangle) -> bool {
	a.top_left.x < b.top_left.x + b.size.width as i32
		&& a.top_left.x + a.size.width as i32 > b.top_left.x
		&& a.top_left.y < b.top_left.y + b.size.height as i32
		&& a.top_left.y + a.size.height as i32 > b.top_left.y
}
