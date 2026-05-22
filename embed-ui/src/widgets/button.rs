use core::str::FromStr;

use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use heapless::String;

use crate::{
	Error,
	input::Interaction,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
	text:    String<MAX_TEXT_LEN>,
	size:    Size,
	focus:   bool,
	changed: bool,
}

impl Button {
	pub fn new(text: &str, size: Size) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			size,
			focus: false,
			changed: true,
		})
	}

	pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}
}

impl Widget for Button {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		let bg = match interaction {
			// Some(Interaction::Hover(p)) => {
			// 	if rect.contains(p) {
			// 		style.active_color
			// 	} else {
			// 		style.bg_color
			// 	}
			// }
			None => style.bg_color,
			_ => todo!(),
		};

		let border_color = match self.focus {
			true => style.focus_color,
			false => style.border_color,
		};

		let prim_style = PrimitiveStyleBuilder::new()
			.stroke_color(border_color)
			.stroke_width(style.border_width)
			.fill_color(bg)
			.build();

		rect.into_styled(prim_style).draw(target)?;

		let ts = TextStyleBuilder::new()
			.alignment(Alignment::Center)
			.baseline(Baseline::Middle)
			.build();

		let text_location = Point::new(
			rect.top_left.x + (rect.size.width / 2) as i32,
			rect.top_left.y + (rect.size.height / 2) as i32,
		);

		Text::with_text_style(
			&self.text,
			text_location,
			MonoTextStyle::new(style.font, style.text_color),
			ts,
		)
		.draw(target)?;

		self.changed = false;

		Ok(())
	}

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_focusable(&self) -> bool {
		true
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
