use embedded_graphics::{
	prelude::{DrawTarget, PixelColor, Point, Size},
	primitives::Rectangle,
};
use embedded_graphics_framebuf::FrameBuf;

use crate::{Error, page::Page, style::Style};

// #[derive(Debug)]
// #[cfg_attr(feature = "defmt", derive(defmt::Format))]
// pub struct FrameBuffer<const N: usize, C: PixelColor> {
// 	pub data: [C; N],
// }

// impl<const N: usize, C: PixelColor> Dimensions for FrameBuffer<N, C> {
// 	fn bounding_box(&self) -> Rectangle {
// 		todo!()
// 	}
// }

// impl<const N: usize, C: PixelColor> DrawTarget for FrameBuffer<N, C> {
// 	type Color = C;
// 	type Error = Error;

// 	fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
// 	where
// 		I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
// 	{
// 		todo!()
// 	}
// }

pub trait Painter {
	fn paint<C: PixelColor, const WIDGET_COUNT: usize, const N: usize>(
		&mut self,
		strip_count: usize,
		style: &Style<C>,
		buffer: &mut FrameBuf<C, [C; N]>,
		page: &mut Page<C, WIDGET_COUNT, N>,
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
}

impl<const STRIP_COUNT: usize, const STRIP_W: usize, const STRIP_H: usize> Painter
	for SplitPainter<STRIP_COUNT, STRIP_W, STRIP_H>
{
	fn paint<C: PixelColor, const WIDGET_COUNT: usize, const N: usize>(
		&mut self,
		strip_count: usize,
		style: &Style<C>,
		buffer: &mut FrameBuf<C, [C; N]>,
		page: &mut Page<C, WIDGET_COUNT, N>,
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
