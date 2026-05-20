use embedded_graphics::{
	mono_font::MonoTextStyle,
	prelude::*,
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use heapless::String;

use crate::{
	Error,
	container::WidgetId,
	style::Style,
	widgets::{MAX_TEXT_LEN, Widget},
};

#[derive(Debug)]
pub struct Button {
	id:      WidgetId,
	text:    String<MAX_TEXT_LEN>,
	size:    Size,
	focus:   bool,
	held:    bool,
	changed: bool,
}

impl Button {
	pub const fn new(id: WidgetId, text: String<MAX_TEXT_LEN>, size: Size) -> Self {
		Self {
			id,
			text,
			size,
			focus: false,
			held: false,
			changed: true,
		}
	}
}

impl Widget for Button {
	fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		rect: impl Drawable<Color = D::Color>,
		target: &mut D,
	) -> Result<(), D::Error> {
		rect.draw(target)?;

		let ts = TextStyleBuilder::new()
			.alignment(Alignment::Center)
			.baseline(Baseline::Middle)
			.build();

		Text::with_text_style(
			&self.text,
			Point::new((self.size.width / 2) as i32, (self.size.height / 2) as i32),
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

	fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}

	fn id(&self) -> WidgetId {
		self.id
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
