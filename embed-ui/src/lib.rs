#![no_std]
#![allow(clippy::new_without_default)]

pub mod button;
pub mod container;
pub mod input;
pub mod style;
pub mod ui;
pub mod widgets;

pub use embedded_graphics::{prelude::*, primitives::Rectangle};
