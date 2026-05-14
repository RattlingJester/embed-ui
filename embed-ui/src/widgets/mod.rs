use embedded_graphics::prelude::DrawTarget;
use enum_dispatch::enum_dispatch;

use crate::{
	style::Style,
	widgets::{button::Button, checkbox::Checkbox, label::Label},
};

pub mod button;
pub mod checkbox;
pub mod label;

#[enum_dispatch]
pub trait Widget {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		target: &mut D,
	) -> Result<(), D::Error>;
}

#[enum_dispatch(Widget)]
#[derive(Debug)]
pub enum WidgetKind<'a> {
	Label(Label<'a>),
	Button(Button<'a>),
	Checkbox(Checkbox<'a>),
}
