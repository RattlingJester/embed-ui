use enum_dispatch::enum_dispatch;

use embedded_graphics::{
	prelude::{DrawTarget, Size},
	primitives::Rectangle,
};

use crate::{
	input::Interaction,
	style::Style,
	widgets::{
		button::Button, checkbox::Checkbox, label::Label, radio_button::RadioButton,
		separator::Separator, textbox::Textbox,
	},
};

pub mod button;
pub mod checkbox;
pub mod label;
pub mod radio_button;
pub mod separator;
pub mod textbox;

pub const MAX_TEXT_LEN: usize = 32;

#[enum_dispatch]
pub trait Widget {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		target: &mut D,
	) -> Result<(), D::Error>;

	fn interact(&mut self, _interaction: Option<Interaction>) {}
	fn set_focus(&mut self, _focus: bool) {}
	fn set_focusable(&mut self, _focusable: bool) {}
	fn mark_clean(&mut self);

	fn size(&self) -> Size;
	fn is_focusable(&self) -> bool {
		false
	}
	fn is_dirty(&self) -> bool;
	fn is_pressed(&self) -> bool {
		false
	}
}

#[enum_dispatch(Widget)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetKind {
	Label(Label),
	Button(Button),
	RadioButton(RadioButton),
	Checkbox(Checkbox),
	Separator(Separator),
	Textbox(Textbox),
}
