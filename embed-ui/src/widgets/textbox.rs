use core::str::FromStr;

use embedded_graphics::{
	Drawable,
	geometry::Point,
	mono_font::{MonoFont, MonoTextStyle},
	prelude::{PixelColor, Size},
	primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
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
pub struct Textbox {
	text:       String<MAX_TEXT_LEN>,
	text_align: TextAlignment,
	font:       &'static MonoFont<'static>,
	size:       Size,
	focus:      bool,
	focusable:  bool,
	changed:    bool,
}

impl Textbox {
	pub fn new(
		text: &str,
		font: &'static MonoFont,
		text_align: TextAlignment,
		size: Size,
		focusable: bool,
	) -> Result<Self, Error> {
		Ok(Self {
			text: String::from_str(text)?,
			text_align,
			font,
			size,
			focus: false,
			focusable,
			changed: true,
		})
	}
}

impl<C: PixelColor, const F: usize> Widget<C, F> for Textbox {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; F]>,
	) -> Result<(), Error> {
		let border_color = match self.focus {
			true => style.focus_color,
			false => style.border_color,
		};

		let border_style = PrimitiveStyleBuilder::new()
			.stroke_color(border_color)
			.stroke_width(style.border_width)
			.fill_color(style.bg_color)
			.build();

		let br = rect.bottom_right().unwrap_or_default();
		let tl = rect.top_left;
		let center = rect.center();

		let padding_x = style.border_width as i32 + 4;
		let padded_left = tl.x + padding_x;
		let padded_right = br.x - padding_x;

		let (horizontal_alignment, vertical_baseline, text_pos) = match self.text_align {
			TextAlignment {
				horizontal: Horizontal::Left,
				vertical: Vertical::Top,
			} => (
				Alignment::Left,
				Baseline::Top,
				Point::new(padded_left, tl.y),
			),
			TextAlignment {
				horizontal: Horizontal::Left,
				vertical: Vertical::Bottom,
			} => (
				Alignment::Left,
				Baseline::Bottom,
				Point::new(padded_left, br.y),
			),
			TextAlignment {
				horizontal: Horizontal::Left,
				vertical: Vertical::Center,
			} => (
				Alignment::Left,
				Baseline::Middle,
				Point::new(padded_left, center.y),
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
			} => (
				Alignment::Right,
				Baseline::Top,
				Point::new(padded_right, tl.y),
			),
			TextAlignment {
				horizontal: Horizontal::Right,
				vertical: Vertical::Bottom,
			} => (
				Alignment::Right,
				Baseline::Bottom,
				Point::new(padded_right, br.y),
			),
			TextAlignment {
				horizontal: Horizontal::Right,
				vertical: Vertical::Center,
			} => (
				Alignment::Right,
				Baseline::Middle,
				Point::new(padded_right, center.y),
			),
		};

		let ts = TextStyleBuilder::new()
			.alignment(horizontal_alignment)
			.baseline(vertical_baseline)
			.build();

		rect.draw_styled(&border_style, target)?;

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

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus && self.focusable {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn set_focusable(&mut self, focusable: bool) {
		self.focusable = focusable
	}

	fn mark_clean(&mut self) {
		self.changed = false
	}

	fn is_focusable(&self) -> bool {
		self.focusable
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
