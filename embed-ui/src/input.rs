use embedded_graphics::prelude::Point;

use crate::container::WidgetId;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
	Click(Point),
	Release(Point),
	Drag(Point),
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Event {
	ButtonClicked { page_idx: u8, widget_id: WidgetId },
	CheckboxToggled { page_idx: u8, widget_id: WidgetId },
}
