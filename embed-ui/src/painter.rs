use embedded_graphics::{
	prelude::{DrawTarget, PixelColor, Point, Size},
	primitives::Rectangle,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{Error, alloc::Allocator, page::Page, style::Style};

pub trait Painter<'a> {
	fn paint<A: Allocator<'a>, C: PixelColor, const WIDGET_COUNT: usize, const N: usize>(
		&mut self,
		strip_count: usize,
		style: &Style<C>,
		buffer: &mut FrameBuf<C, [C; N]>,
		page: &mut Page<'a, C, A, WIDGET_COUNT, N>,
	) -> Result<Rectangle, Error>;
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct SplitPainter<const STRIP_COUNT: usize, const STRIP_W: usize, const STRIP_H: usize> {}

impl<const STRIP_COUNT: usize, const STRIP_W: usize, const STRIP_H: usize>
	SplitPainter<STRIP_COUNT, STRIP_W, STRIP_H>
{
	#[allow(clippy::new_without_default)]
	pub const fn new() -> Self {
		Self {}
	}

	/// Returns a bitmask of strips
	/// Bit N is set if strip N needs repainting.
	pub fn dirty_strip_mask<
		'a,
		A: Allocator<'a>,
		C: PixelColor,
		const WIDGET_COUNT: usize,
		const N: usize,
	>(
		&self,
		page: &Page<'a, C, A, WIDGET_COUNT, N>,
		strip_h: usize,
		strip_count: usize,
	) -> u32 {
		let mut mask = 0u32;
		for entry in page.widgets[..page.count].iter().flatten() {
			let (widget, rect) = entry;
			if widget.is_changed() {
				let y0 = rect.top_left.y.max(0) as usize;
				let y1 = (y0 + rect.size.height as usize).saturating_sub(1);
				let first = y0 / strip_h;
				let last = (y1 / strip_h).min(strip_count - 1);
				for s in first..=last {
					mask |= 1 << s;
				}
			}
		}
		mask
	}
}

impl<'a, const STRIP_COUNT: usize, const STRIP_W: usize, const STRIP_H: usize> Painter<'a>
	for SplitPainter<STRIP_COUNT, STRIP_W, STRIP_H>
{
	fn paint<A: Allocator<'a>, C: PixelColor, const WIDGET_COUNT: usize, const N: usize>(
		&mut self,
		strip_count: usize,
		style: &Style<C>,
		buffer: &mut FrameBuf<C, [C; N]>,
		page: &mut Page<'a, C, A, WIDGET_COUNT, N>,
	) -> Result<Rectangle, Error> {
		let y0 = strip_count * STRIP_H;
		let strip_rect = Rectangle::new(
			Point::new(0, y0 as i32),
			Size::new(STRIP_W as u32, STRIP_H as u32),
		);

		buffer.clear(style.screen_bg);

		for (widget, rect) in page.widgets[..page.count].iter_mut().flatten() {
			if !rect.intersection(&strip_rect).is_zero_sized() {
				let mut shifted_rect = *rect;
				shifted_rect.top_left.y -= y0 as i32;

				widget.draw(style, &shifted_rect, buffer)?;
			}
		}

		Ok(strip_rect)
	}
}
