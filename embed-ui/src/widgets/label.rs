use core::str::FromStr;

use embedded_graphics::{
	Drawable,
	mono_font::{MonoFont, MonoTextStyle},
	prelude::{PixelColor, Point, Size},
	primitives::Rectangle,
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};

use embedded_graphics_framebuf::FrameBuf;
use heapless::String;

use crate::{
	Error,
	style::Style,
	widgets::{Horizontal, MAX_TEXT_LEN, TextAlignment, Vertical, Widget},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub struct Label {
	text:       String<MAX_TEXT_LEN>,
	text_align: TextAlignment,
	font:       &'static MonoFont<'static>,
	size:       Size,
	changed:    bool,
}

impl Label {
	pub fn new(
		text: &str,
		font: &'static MonoFont,
		text_align: TextAlignment,
		size: Size,
	) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			font,
			text_align,
			size,
			changed: true,
		})
	}
}

impl<C: PixelColor, const F: usize> Widget<C, F> for Label {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; F]>,
	) -> Result<(), Error> {
		let br = rect.bottom_right().unwrap_or_default();
		let tl = rect.top_left;
		let center = rect.center();

		let (horizontal_alignment, vertical_baseline, text_pos) = match self.text_align {
			TextAlignment {
				horizontal: Horizontal::Left,
				vertical: Vertical::Top,
			} => (Alignment::Left, Baseline::Top, Point::new(tl.x, tl.y)),
			TextAlignment {
				horizontal: Horizontal::Left,
				vertical: Vertical::Bottom,
			} => (Alignment::Left, Baseline::Bottom, Point::new(tl.x, br.y)),
			TextAlignment {
				horizontal: Horizontal::Left,
				vertical: Vertical::Center,
			} => (
				Alignment::Left,
				Baseline::Middle,
				Point::new(tl.x, center.y),
			),

			TextAlignment {
				horizontal: Horizontal::Center,
				vertical: Vertical::Top,
			} => (Alignment::Center, Baseline::Top, Point::new(center.x, tl.y)),
			TextAlignment {
				horizontal: Horizontal::Center,
				vertical: Vertical::Bottom,
			} => (
				Alignment::Center,
				Baseline::Bottom,
				Point::new(center.x, br.y),
			),
			TextAlignment {
				horizontal: Horizontal::Center,
				vertical: Vertical::Center,
			} => (Alignment::Center, Baseline::Middle, center),

			TextAlignment {
				horizontal: Horizontal::Right,
				vertical: Vertical::Top,
			} => (Alignment::Right, Baseline::Top, Point::new(br.x, tl.y)),
			TextAlignment {
				horizontal: Horizontal::Right,
				vertical: Vertical::Bottom,
			} => (Alignment::Right, Baseline::Bottom, Point::new(br.x, br.y)),
			TextAlignment {
				horizontal: Horizontal::Right,
				vertical: Vertical::Center,
			} => (
				Alignment::Right,
				Baseline::Middle,
				Point::new(br.x, center.y),
			),
		};

		let ts = TextStyleBuilder::new()
			.alignment(horizontal_alignment)
			.baseline(vertical_baseline)
			.build();

		Text::with_text_style(
			&self.text,
			text_pos,
			MonoTextStyle::new(self.font, style.text_color),
			ts,
		)
		.draw(target)?;
		Ok(())
	}

	fn set_text(&mut self, text: &str) -> Result<(), Error> {
		self.text.clear();
		self.text.push_str(text)?;
		self.changed = true;

		Ok(())
	}

	fn mark_clean(&mut self) {
		self.changed = false;
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_changed(&self) -> bool {
		self.changed
	}

	fn set_focus(&mut self, _focus: bool) {}

	fn set_focusable(&mut self, _focusable: bool) {}

	fn is_focusable(&self) -> bool {
		false
	}
}
