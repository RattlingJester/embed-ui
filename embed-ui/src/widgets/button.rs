use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};

use crate::{style::Style, widgets::Widget};

#[derive(Debug)]
pub struct Button<'a> {
	pub text:    &'a str,
	pub bounds:  Rectangle,
	pub focus:   bool,
	pub held:    bool,
	pub changed: bool,
}

impl<'a> Button<'a> {
	pub const fn new(text: &'a str, bounds: Rectangle) -> Self {
		Self {
			text,
			bounds,
			focus: false,
			held: false,
			changed: true,
		}
	}
}

impl Widget for Button<'_> {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		target: &mut D,
	) -> Result<(), <D as DrawTarget>::Error> {
		let bg = if self.held {
			style.active_color
		} else {
			style.bg_color
		};

		let border = if self.focus {
			style.focus_color
		} else {
			style.border_color
		};

		let prim_style = PrimitiveStyleBuilder::new()
			.stroke_color(border)
			.stroke_width(style.border_width)
			.fill_color(bg)
			.build();

		self.bounds.into_styled(prim_style).draw(target)?;

		let ts = TextStyleBuilder::new()
			.alignment(Alignment::Center)
			.baseline(Baseline::Middle)
			.build();

		Text::with_text_style(
			self.text,
			self.bounds.center(),
			MonoTextStyle::new(style.font, style.text_color),
			ts,
		)
		.draw(target)?;

		self.changed = false;

		Ok(())
	}

	fn update(&mut self) {
		// self.changed = true;
		todo!()
	}

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
