use embedded_graphics::{prelude::PixelColor, primitives::Rectangle};
use embedded_graphics_framebuf::FrameBuf;

use heapless::spsc::Queue;

use crate::{
	Error,
	alloc::Allocator,
	input::{Event, Interaction},
	page::Page,
	painter::Painter,
	style::Style,
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ui<
	'a,
	const WIDGET_COUNT: usize,
	const PAGE_COUNT: usize,
	const FB_SIZE: usize,
	A: Allocator<'a>,
	C: PixelColor,
	P: Painter<'a>,
> {
	pages:           [Page<'a, C, A, WIDGET_COUNT, FB_SIZE>; PAGE_COUNT],
	active_page_idx: u8,
	events:          Queue<Event, WIDGET_COUNT>,
	pub painter:     P,
	pub style:       Style<C>,
}

impl<
	'a,
	const WIDGET_COUNT: usize,
	const PAGE_COUNT: usize,
	const FB_SIZE: usize,
	A: Allocator<'a>,
	C: PixelColor,
	P: Painter<'a>,
> Ui<'a, WIDGET_COUNT, PAGE_COUNT, FB_SIZE, A, C, P>
{
	pub const fn new(
		pages: [Page<'a, C, A, WIDGET_COUNT, FB_SIZE>; PAGE_COUNT],
		painter: P,
		style: Style<C>,
	) -> Self {
		Self {
			pages,
			events: Queue::new(),
			painter,
			active_page_idx: 0,
			style,
		}
	}

	pub fn drain_events(&mut self) -> Option<Event> {
		self.events.dequeue()
	}

	pub fn switch_to_page(&mut self, idx: u8) -> bool {
		if idx < PAGE_COUNT as u8 {
			self.active_page_idx = idx;
			true
		} else {
			false
		}
	}

	/// Switches to the next page, wrapping back to the first page if at the end.
	pub fn next_page(&mut self) {
		self.active_page_idx = (self.active_page_idx + 1) % PAGE_COUNT as u8;
	}

	/// Switches to the previous page, wrapping to the last page if at the beginning.
	pub fn prev_page(&mut self) {
		self.active_page_idx = if self.active_page_idx == 0 {
			PAGE_COUNT as u8 - 1
		} else {
			self.active_page_idx - 1
		};
	}

	/// Retrieves the currently active page immutably
	pub fn current_page(&self) -> &Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&self.pages[self.active_page_idx as usize]
	}

	/// Retrieves the currently active page mutably
	pub fn current_page_mut(&mut self) -> &mut Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&mut self.pages[self.active_page_idx as usize]
	}

	pub fn get_page(&self, idx: u8) -> &Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&self.pages[idx as usize]
	}

	pub fn get_page_mut(&mut self, idx: u8) -> &mut Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&mut self.pages[idx as usize]
	}

	/// Processes user interactions. Need to call this function before drawing each frame
	pub fn begin_frame(&mut self, interaction: Option<Interaction>) {
		let page = &mut self.pages[self.active_page_idx as usize];

		page.process(interaction, &mut self.events, self.active_page_idx);
	}

	pub fn draw(
		&mut self,
		strip_count: usize,
		buffer: &mut FrameBuf<C, [C; FB_SIZE]>,
	) -> Result<Rectangle, Error> {
		let page = &mut self.pages[self.active_page_idx as usize];

		let rect = self.painter.paint(strip_count, &self.style, buffer, page)?;

		Ok(rect)
	}

	/// Flags all widget as clean (rendered) so you won't have to re-render them on the next frame if they don't change. Have to call this function after drawing each frame
	pub fn end_frame(&mut self) {
		let page = &mut self.pages[self.active_page_idx as usize];

		for (w, _rect) in page.iter_mut() {
			w.mark_clean();
		}
	}
}
