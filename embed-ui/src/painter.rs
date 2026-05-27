use embedded_graphics::{
	prelude::{DrawTarget, DrawTargetExt, PixelColor, Point, Size},
	primitives::Rectangle,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{container::Page, style::Style, widgets::Widget};

pub trait Painter<C: PixelColor, const N: usize> {
	fn draw<const WIDGET_COUNT: usize, D: DrawTarget<Color = C>>(
		&mut self,
		style: &Style<C>,
		page: &mut Page<WIDGET_COUNT>,
		target: &mut D,
	) -> Result<(), D::Error>;

	fn data_mut(&mut self) -> FrameBuf<C, &mut [C; N]>;
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct SplitPainter<
	const STRIP_COUNT: usize,
	const STRIP_W: usize,
	const STRIP_H: usize,
	const N: usize,
	C: PixelColor,
> {
	pub buffer: [C; N],
}

impl<
	const STRIP_COUNT: usize,
	const STRIP_W: usize,
	const STRIP_H: usize,
	const N: usize,
	C: PixelColor,
> SplitPainter<STRIP_COUNT, STRIP_W, STRIP_H, N, C>
{
	pub const fn new(buffer: [C; N]) -> Self {
		Self { buffer }
	}
}

impl<
	const STRIP_COUNT: usize,
	const STRIP_W: usize,
	const STRIP_H: usize,
	const N: usize,
	C: PixelColor,
> Painter<C, N> for SplitPainter<STRIP_COUNT, STRIP_W, STRIP_H, N, C>
{
	fn draw<const WIDGET_COUNT: usize, D: DrawTarget<Color = C>>(
		&mut self,
		style: &Style<C>,
		page: &mut Page<WIDGET_COUNT>,
		target: &mut D,
	) -> Result<(), D::Error> {
		for strip in 0..STRIP_COUNT {
			let y0 = strip * STRIP_H;

			let mut buf = FrameBuf::new(&mut self.buffer, STRIP_W, STRIP_H);

			buf.clear(style.screen_bg);

			let strip_rect = Rectangle::new(
				Point::new(0, y0 as i32),
				Size::new(STRIP_W as u32, STRIP_H as u32),
			);

			let mut translated = buf.translated(Point::new(0, -strip_rect.top_left.y));

			for (widget, rect) in page.widgets[..page.count].iter_mut().flatten() {
				if !rect.intersection(&strip_rect).is_zero_sized() {
					widget.draw(style, rect, &mut translated);
				}
			}

			target.fill_contiguous(&strip_rect, self.buffer)?;
		}

		Ok(())
	}

	fn data_mut(&mut self) -> FrameBuf<C, &mut [C; N]> {
		FrameBuf::new(&mut self.buffer, STRIP_W, STRIP_H)
	}
}
