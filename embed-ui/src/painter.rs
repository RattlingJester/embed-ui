use embedded_graphics::{
	prelude::{DrawTarget, DrawTargetExt, PixelColor, Point, Size},
	primitives::Rectangle,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{container::Page, style::Style, widgets::Widget};

pub trait Painter<C: PixelColor, const N: usize> {
	async fn draw<
		const WIDGET_COUNT: usize,
		D: DrawTarget<Color = C>,
		F: AsyncFnMut(&Rectangle, &mut FrameBuf<C, [C; N]>) -> Result<(), D::Error>,
	>(
		&mut self,
		style: &Style<C>,
		page: &mut Page<WIDGET_COUNT>,
		finish: F,
	) -> Result<(), D::Error>;

	fn data_mut(&mut self) -> FrameBuf<C, [C; N]>;
}

pub struct SplitPainter<
	'a,
	const STRIP_COUNT: usize,
	const STRIP_W: usize,
	const STRIP_H: usize,
	const N: usize,
	C: PixelColor,
> {
	pub buffer: &'a mut [C; N],
}

impl<
	'a,
	const STRIP_COUNT: usize,
	const STRIP_W: usize,
	const STRIP_H: usize,
	const N: usize,
	C: PixelColor,
> SplitPainter<'a, STRIP_COUNT, STRIP_W, STRIP_H, N, C>
{
	pub const fn new(buffer: &'a mut [C; N]) -> Self {
		Self { buffer }
	}
}

impl<
	'a,
	const STRIP_COUNT: usize,
	const STRIP_W: usize,
	const STRIP_H: usize,
	const N: usize,
	C: PixelColor,
> Painter<C, N> for SplitPainter<'a, STRIP_COUNT, STRIP_W, STRIP_H, N, C>
{
	async fn draw<
		const WIDGET_COUNT: usize,
		D: DrawTarget<Color = C>,
		F: AsyncFnMut(&Rectangle, &mut FrameBuf<C, [C; N]>) -> Result<(), D::Error>,
	>(
		&mut self,
		style: &Style<C>,
		page: &mut Page<WIDGET_COUNT>,
		mut finish: F,
	) -> Result<(), D::Error> {
		for strip in 0..STRIP_COUNT {
			let y0 = strip * STRIP_H;

			let mut buf = FrameBuf::new(*self.buffer, STRIP_W, STRIP_H);

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

			finish(&strip_rect, &mut buf).await?;

			// target.fill_contiguous(&strip_rect, self.buffer)?;
		}

		Ok(())
	}

	fn data_mut(&mut self) -> FrameBuf<C, [C; N]> {
		FrameBuf::new(*self.buffer, STRIP_W, STRIP_H)
	}
}
