use enum_dispatch::enum_dispatch;

use embedded_graphics::{
	prelude::{DrawTarget, Size},
	primitives::Rectangle,
};

use crate::{
	Error,
	input::Interaction,
	style::Style,
	widgets::{button::Button, checkbox::Checkbox, label::Label},
};

pub mod button;
pub mod checkbox;
pub mod label;

pub const MAX_TEXT_LEN: usize = 64;

#[enum_dispatch]
pub trait Widget {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,

		rect: &Rectangle,
		interaction: Option<Interaction>,

		target: &mut D,
	) -> Result<(), D::Error>;

	fn set_focus(&mut self, focus: bool);
	fn set_text(&mut self, text: &str) -> Result<(), Error>;

	fn size(&self) -> Size;
	fn is_changed(&self) -> bool;
}

#[enum_dispatch(Widget)]
#[derive(Debug)]
pub enum WidgetKind {
	Label(Label),
	Button(Button),
	Checkbox(Checkbox),
}
