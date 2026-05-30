use embedded_graphics::prelude::Point;

use crate::page::WidgetId;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
	Click(Point),
	Release(Point),
	Drag(Point),
}

impl Interaction {
	pub fn point(&self) -> Point {
		match self {
			Self::Click(p) => *p,
			Self::Release(p) => *p,
			Self::Drag(p) => *p,
		}
	}
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Event {
	ButtonClicked { page_idx: u8, widget_id: WidgetId },
	CheckboxToggled { page_idx: u8, widget_id: WidgetId },
}
