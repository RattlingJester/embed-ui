use enum_dispatch::enum_dispatch;

use embedded_graphics::{
	prelude::{DrawTarget, Size},
	primitives::Rectangle,
};

use crate::{
	input::Interaction,
	style::Style,
	widgets::{
		button::Button, checkbox::Checkbox, label::Label, separator::Separator, textbox::Textbox,
	},
};

pub mod button;
pub mod checkbox;
pub mod label;
pub mod separator;
pub mod textbox;

pub const MAX_TEXT_LEN: usize = 64;

#[enum_dispatch]
pub(crate) trait Widget {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		target: &mut D,
	) -> Result<(), D::Error>;

	fn interact(&mut self, rect: &Rectangle, interaction: Option<Interaction>);
	fn set_focus(&mut self, focus: bool);
	fn mark_clean(&mut self);

	fn size(&self) -> Size;
	fn is_focusable(&self) -> bool;
	fn is_dirty(&self) -> bool;
	fn is_pressed(&self) -> bool;
}

#[enum_dispatch(Widget)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetKind {
	Label(Label),
	Button(Button),
	Checkbox(Checkbox),
	Separator(Separator),
	Textbox(Textbox),
}
