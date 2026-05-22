use core::str::FromStr;

use embedded_graphics::{
	Drawable,
	mono_font::MonoTextStyle,
	prelude::{DrawTarget, Point, Size},
	primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
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
pub struct Textbox {
	text:    String<MAX_TEXT_LEN>,
	size:    Size,
	changed: bool,
}

impl Textbox {
	pub fn new(text: &str, size: Size) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			size,
			changed: true,
		})
	}
}

impl Widget for Textbox {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: &Rectangle,
		_interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		let border_style = PrimitiveStyleBuilder::new()
			.stroke_color(style.border_color)
			.stroke_width(style.border_width)
			.fill_color(style.bg_color)
			.build();

		let padding_x = 4;

		let ts = TextStyleBuilder::new()
			.alignment(Alignment::Left)
			.baseline(Baseline::Middle)
			.build();

		let text_pos = Point::new(
			rect.top_left.x + padding_x,
			rect.top_left.y + rect.size.height as i32 / 2,
		);

		rect.draw_styled(&border_style, target)?;

		Text::with_text_style(
			&self.text,
			text_pos,
			MonoTextStyle::new(style.font, style.text_color),
			ts,
		)
		.draw(target)?;

		Ok(())
	}

	fn set_focus(&mut self, _focus: bool) {}

	fn size(&self) -> Size {
		self.size
	}

	fn is_focusable(&self) -> bool {
		false
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
