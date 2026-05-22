use embedded_graphics::prelude::Point;

use crate::container::WidgetId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
	Click(Point),
	Release(Point),
	Drag(Point),
}

#[derive(Debug)]
pub enum Event {
	ButtonClicked { page_idx: u8, widget_id: WidgetId },
	CheckboxToggled { page_idx: u8, widget_id: WidgetId },
}
