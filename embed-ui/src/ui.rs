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
	pages:               [Page<'a, C, A, WIDGET_COUNT, FB_SIZE>; PAGE_COUNT],
	events:              Queue<Event, WIDGET_COUNT>,
	pub painter:         P,
	pub active_page_idx: u8,
	pub dirty_frame:     bool,
	pub style:           Style<C>,
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
			dirty_frame: true,
			style,
		}
	}

	pub fn drain_events(&mut self) -> Option<Event> {
		self.events.dequeue()
	}

	pub fn switch_to_page(&mut self, idx: u8) -> bool {
		if idx < PAGE_COUNT as u8 {
			self.dirty_frame = true;
			self.active_page_idx = idx;
			true
		} else {
			false
		}
	}

	/// Switches to the next page, wrapping back to the first page if at the end.
	pub fn next_page(&mut self) {
		self.active_page_idx = (self.active_page_idx + 1) % PAGE_COUNT as u8;
		self.dirty_frame = true;
	}

	/// Switches to the previous page, wrapping to the last page if at the beginning.
	pub fn prev_page(&mut self) {
		self.active_page_idx = if self.active_page_idx == 0 {
			PAGE_COUNT as u8 - 1
		} else {
			self.active_page_idx - 1
		};
		self.dirty_frame = true;
	}

	/// Retrieves the currently active page immutably
	pub fn current_page(&self) -> &Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&self.pages[self.active_page_idx as usize]
	}

	/// Retrieves the currently active page mutably
	pub fn current_page_mut(&mut self) -> &mut Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&mut self.pages[self.active_page_idx as usize]
	}

	// pub fn get_button(&self, page_idx: u8, id: WidgetId) -> Option<(&Button, &Rectangle)> {
	// 	if let (WidgetKind::Button(b), rect) = self.get_page(page_idx).get(id)? {
	// 		Some((b, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_button_mut(
	// 	&mut self,
	// 	page_idx: u8,
	// 	id: WidgetId,
	// ) -> Option<(&mut Button, &mut Rectangle)> {
	// 	if let (WidgetKind::Button(b), rect) = self.get_page_mut(page_idx).get_mut(id)? {
	// 		Some((b, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_label(&self, page_idx: u8, id: WidgetId) -> Option<(&Label, &Rectangle)> {
	// 	if let (WidgetKind::Label(l), rect) = self.get_page(page_idx).get(id)? {
	// 		Some((l, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_label_mut(
	// 	&mut self,
	// 	page_idx: u8,
	// 	id: WidgetId,
	// ) -> Option<(&mut Label, &mut Rectangle)> {
	// 	if let (WidgetKind::Label(l), rect) = self.get_page_mut(page_idx).get_mut(id)? {
	// 		Some((l, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_checkbox(&self, page_idx: u8, id: WidgetId) -> Option<(&Checkbox, &Rectangle)> {
	// 	if let (WidgetKind::Checkbox(c), rect) = self.get_page(page_idx).get(id)? {
	// 		Some((c, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_checkbox_mut(
	// 	&mut self,
	// 	page_idx: u8,
	// 	id: WidgetId,
	// ) -> Option<(&mut Checkbox, &mut Rectangle)> {
	// 	if let (WidgetKind::Checkbox(c), rect) = self.get_page_mut(page_idx).get_mut(id)? {
	// 		Some((c, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_separator(&self, page_idx: u8, id: WidgetId) -> Option<(&Separator, &Rectangle)> {
	// 	if let (WidgetKind::Separator(s), rect) = self.get_page(page_idx).get(id)? {
	// 		Some((s, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_separator_mut(
	// 	&mut self,
	// 	page_idx: u8,
	// 	id: WidgetId,
	// ) -> Option<(&mut Separator, &mut Rectangle)> {
	// 	if let (WidgetKind::Separator(s), rect) = self.get_page_mut(page_idx).get_mut(id)? {
	// 		Some((s, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_textbox(&self, page_idx: u8, id: WidgetId) -> Option<(&Textbox, &Rectangle)> {
	// 	if let (WidgetKind::Textbox(t), rect) = self.get_page(page_idx).get(id)? {
	// 		Some((t, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	// pub fn get_textbox_mut(
	// 	&mut self,
	// 	page_idx: u8,
	// 	id: WidgetId,
	// ) -> Option<(&mut Textbox, &mut Rectangle)> {
	// 	if let (WidgetKind::Textbox(t), rect) = self.get_page_mut(page_idx).get_mut(id)? {
	// 		Some((t, rect))
	// 	} else {
	// 		None
	// 	}
	// }

	pub fn get_page(&self, idx: u8) -> &Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&self.pages[idx as usize]
	}

	pub fn get_page_mut(&mut self, idx: u8) -> &mut Page<'a, C, A, WIDGET_COUNT, FB_SIZE> {
		&mut self.pages[idx as usize]
	}

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

	pub fn end_frame(&mut self) {
		self.dirty_frame = false;

		let page = &mut self.pages[self.active_page_idx as usize];

		for (w, _rect) in page.iter_mut() {
			w.mark_clean();
		}
	}
}
