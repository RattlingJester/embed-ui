use embedded_graphics::prelude::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
	Click(Point),
	Release(Point),
	Drag(Point),
	Hover(Point),
}
