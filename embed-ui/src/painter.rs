use core::marker::PhantomData;

use embedded_graphics::{
	prelude::{DrawTarget, DrawTargetExt, PixelColor, Point, Size},
	primitives::Rectangle,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{Error, container::Page, style::Style, widgets::Widget};

pub trait Painter<C: PixelColor> {
	fn paint<const WIDGET_COUNT: usize, const N: usize>(
		&mut self,
		strip_count: usize,
		style: &Style<C>,
		buffer: &mut [C; N],
		page: &mut Page<WIDGET_COUNT>,
	) -> Result<Rectangle, Error>;
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct SplitPainter<
	const STRIP_COUNT: usize,
	const STRIP_W: usize,
	const STRIP_H: usize,
	C: PixelColor,
> {
	// pub buffer: BUF,
	_phantom: PhantomData<C>,
}

impl<const STRIP_COUNT: usize, const STRIP_W: usize, const STRIP_H: usize, C: PixelColor>
	SplitPainter<STRIP_COUNT, STRIP_W, STRIP_H, C>
{
	#[allow(clippy::new_without_default)]
	pub const fn new() -> Self {
		Self {
			// buffer,
			_phantom: PhantomData,
		}
	}
}

impl<const STRIP_COUNT: usize, const STRIP_W: usize, const STRIP_H: usize, C: PixelColor> Painter<C>
	for SplitPainter<STRIP_COUNT, STRIP_W, STRIP_H, C>
{
	fn paint<const WIDGET_COUNT: usize, const N: usize>(
		&mut self,
		strip_count: usize,
		style: &Style<C>,
		buffer: &mut [C; N],
		page: &mut Page<WIDGET_COUNT>,
	) -> Result<Rectangle, Error> {
		let y0 = strip_count * STRIP_H;
		let strip_rect = Rectangle::new(
			Point::new(0, y0 as i32),
			Size::new(STRIP_W as u32, STRIP_H as u32),
		);

		let mut buf = FrameBuf::new(buffer, STRIP_W, STRIP_H);

		buf.clear(style.screen_bg);

		let mut translated = buf.translated(Point::new(0, -strip_rect.top_left.y));

		for (widget, rect) in page.widgets[..page.count].iter_mut().flatten() {
			if !rect.intersection(&strip_rect).is_zero_sized() {
				widget.draw(style, rect, &mut translated);
			}
		}

		Ok(strip_rect)
	}
}
