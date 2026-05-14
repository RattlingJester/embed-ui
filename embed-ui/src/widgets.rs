use crate::{button::Button, style::Style};

pub trait Widget {
	fn draw<D: DrawTarget>(&mut self, style: &Style<D::Color>, target: D) -> Result<(), D::Error>;
}

#[derive(Debug)]
pub enum WidgetKind<'a> {
	Label,
	Button(Button<'a>),
	Checkbox,
}
