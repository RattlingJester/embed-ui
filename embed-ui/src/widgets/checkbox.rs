use core::str::FromStr;

use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};

use heapless::String;

use crate::{
	input::Interaction,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[derive(Debug)]
pub struct Checkbox {
	text:    String<MAX_TEXT_LEN>,
	size:    Size,
	focus:   bool,
	checked: bool,
	changed: bool,
}

impl Checkbox {
	pub fn new(text: &str, size: Size) -> Self {
		Self {
			text: String::from_str(text).unwrap(),
			size,
			focus: false,
			checked: false,
			changed: true,
		}
	}
}

impl Widget for Checkbox {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		let bg = match interaction {
			None => style.bg_color,
			_ => todo!(),
		};

		let border_color = match self.focus {
			true => style.focus_color,
			false => style.border_color,
		};

		let border_style = PrimitiveStyleBuilder::new()
			.stroke_color(border_color)
			.stroke_width(style.border_width)
			.fill_color(bg)
			.build();

		if self.checked {
			let shortest = rect.size.width.min(rect.size.height);
			let pad = (shortest / 4).max(2) as i32;
			let inner = Rectangle::new(
				rect.top_left + Point::new(pad, pad),
				Size::new(
					rect.size.width.saturating_sub(pad as u32 * 2),
					rect.size.height.saturating_sub(pad as u32 * 2),
				),
			);

			let fill_style = PrimitiveStyleBuilder::new()
				.fill_color(style.active_color)
				.build();

			inner.into_styled(fill_style).draw(target)?;
		}

		let ts = TextStyleBuilder::new()
			.alignment(Alignment::Left)
			.baseline(Baseline::Middle)
			.build();

		let text_location = Point::new(
			rect.top_left.x + (rect.size.width + style.border_width) as i32,
			rect.top_left.y + (rect.size.height / 2) as i32,
		);

		Text::with_text_style(
			&self.text,
			text_location,
			MonoTextStyle::new(style.font, style.text_color),
			ts,
		)
		.draw(target)?;

		rect.into_styled(border_style).draw(target)?;

		self.changed = false;

		Ok(())
	}

	fn size(&self) -> Size {
		self.size
	}

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn set_text(&mut self, text: &str) -> Result<(), crate::Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
