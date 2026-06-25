use core::str::FromStr;

use embedded_graphics::{
	mono_font::{MonoFont, MonoTextStyle},
	prelude::*,
	primitives::{PrimitiveStyleBuilder, Rectangle},
	text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_framebuf::FrameBuf;
use heapless::String;

use crate::{
	Error,
	input::Interaction,
	style::Style,
	widgets::{Horizontal, MAX_TEXT_LEN, TextAlignment, Vertical, Widget},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub struct Button {
	text:       String<MAX_TEXT_LEN>,
	text_align: TextAlignment,
	font:       &'static MonoFont<'static>,
	size:       Size,
	focus:      bool,
	focusable:  bool,
	changed:    bool,
	pressed:    bool,
}

impl Button {
	pub fn new(
		text: &str,
		font: &'static MonoFont<'static>,
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
			pressed: false,
		})
	}
}

impl<C: PixelColor, const F: usize> Widget<C, F> for Button {
	fn draw(
		&mut self,
		style: &Style<C>,
		rect: &Rectangle,
		target: &mut FrameBuf<C, [C; F]>,
	) -> Result<(), Error> {
		let bg = if self.pressed {
			style.active_color
		} else {
			style.bg_color
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

		rect.into_styled(prim_style).draw(target)?;

		Text::with_text_style(
			&self.text,
			text_pos,
			MonoTextStyle::new(self.font, style.text_color),
			ts,
		)
		.draw(target)?;

		Ok(())
	}

	fn interact(&mut self, interaction: Option<Interaction>) {
		match interaction {
			Some(Interaction::Click(_)) if !self.pressed => {
				self.pressed = true;
				self.changed = true;
			}
			Some(Interaction::Release(_)) if self.pressed => {
				self.pressed = false;
				self.changed = true;
			}
			None if self.pressed => {
				self.pressed = false;
				self.changed = true;
			}
			_ => (),
		}
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

	fn set_focus(&mut self, focus: bool) {
		if self.focus != focus && self.focusable {
			self.changed = true;
			self.focus = focus;
		}
	}

	fn set_focusable(&mut self, focusable: bool) {
		self.focusable = focusable
	}

	fn size(&self) -> Size {
		self.size
	}

	fn is_pressed(&self) -> bool {
		self.pressed
	}

	fn is_focusable(&self) -> bool {
		self.focusable
	}

	fn is_changed(&self) -> bool {
		self.changed
	}
}
