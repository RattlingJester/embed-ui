use embedded_graphics::prelude::{DrawTarget, PixelColor};
use heapless::spsc::Queue;

use crate::{
	container::{Page, WidgetId},
	input::{Event, Interaction},
	painter::Painter,
	style::Style,
	widgets::{
		Widget, WidgetKind, button::Button, checkbox::Checkbox, label::Label, separator::Separator,
		textbox::Textbox,
	},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct Ui<const WIDGET_COUNT: usize, const PAGE_COUNT: usize, C: PixelColor, P: Painter<C>> {
	pages:           [Page<WIDGET_COUNT>; PAGE_COUNT],
	events:          Queue<Event, WIDGET_COUNT>,
	painter:         P,
	active_page_idx: u8,
	dirty_frame:     bool,
	pub style:       Style<C>,
}

impl<const WIDGET_COUNT: usize, const PAGE_COUNT: usize, C: PixelColor, P: Painter<C>>
	Ui<WIDGET_COUNT, PAGE_COUNT, C, P>
{
	pub const fn new(pages: [Page<WIDGET_COUNT>; PAGE_COUNT], painter: P, style: Style<C>) -> Self {
		Self {
			pages,
			events: Queue::new(),
			painter,
			active_page_idx: 0,
			dirty_frame: false,
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
	pub fn current_page(&self) -> &Page<WIDGET_COUNT> {
		&self.pages[self.active_page_idx as usize]
	}

	/// Retrieves the currently active page mutably
	pub fn current_page_mut(&mut self) -> &mut Page<WIDGET_COUNT> {
		&mut self.pages[self.active_page_idx as usize]
	}

	pub fn get_button(&self, page_idx: u8, id: WidgetId) -> Option<&Button> {
		if let Some(WidgetKind::Button(b)) = self.get_page(page_idx).get(id) {
			Some(b)
		} else {
			None
		}
	}

	pub fn get_button_mut(&mut self, page_idx: u8, id: WidgetId) -> Option<&mut Button> {
		if let Some(WidgetKind::Button(b)) = self.get_page_mut(page_idx).get_mut(id) {
			Some(b)
		} else {
			None
		}
	}

	pub fn get_label(&self, page_idx: u8, id: WidgetId) -> Option<&Label> {
		if let WidgetKind::Label(l) = self.get_page(page_idx).get(id)? {
			Some(l)
		} else {
			None
		}
	}

	pub fn get_label_mut(&mut self, page_idx: u8, id: WidgetId) -> Option<&mut Label> {
		if let WidgetKind::Label(l) = self.get_page_mut(page_idx).get_mut(id)? {
			Some(l)
		} else {
			None
		}
	}

	pub fn get_checkbox(&self, page_idx: u8, id: WidgetId) -> Option<&Checkbox> {
		if let WidgetKind::Checkbox(c) = self.get_page(page_idx).get(id)? {
			Some(c)
		} else {
			None
		}
	}

	pub fn get_checkbox_mut(&mut self, page_idx: u8, id: WidgetId) -> Option<&mut Checkbox> {
		if let WidgetKind::Checkbox(c) = self.get_page_mut(page_idx).get_mut(id)? {
			Some(c)
		} else {
			None
		}
	}

	pub fn get_separator(&self, page_idx: u8, id: WidgetId) -> Option<&Separator> {
		if let WidgetKind::Separator(s) = self.get_page(page_idx).get(id)? {
			Some(s)
		} else {
			None
		}
	}

	pub fn get_separator_mut(&mut self, page_idx: u8, id: WidgetId) -> Option<&mut Separator> {
		if let WidgetKind::Separator(s) = self.get_page_mut(page_idx).get_mut(id)? {
			Some(s)
		} else {
			None
		}
	}

	pub fn get_textbox(&self, page_idx: u8, id: WidgetId) -> Option<&Textbox> {
		if let WidgetKind::Textbox(t) = self.get_page(page_idx).get(id)? {
			Some(t)
		} else {
			None
		}
	}

	pub fn get_textbox_mut(&mut self, page_idx: u8, id: WidgetId) -> Option<&mut Textbox> {
		if let WidgetKind::Textbox(t) = self.get_page_mut(page_idx).get_mut(id)? {
			Some(t)
		} else {
			None
		}
	}

	pub fn get_page(&self, idx: u8) -> &Page<WIDGET_COUNT> {
		&self.pages[idx as usize]
	}

	pub fn get_page_mut(&mut self, idx: u8) -> &mut Page<WIDGET_COUNT> {
		&mut self.pages[idx as usize]
	}

	pub fn draw<D: DrawTarget<Color = C>>(
		&mut self,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		let page = &mut self.pages[self.active_page_idx as usize];

		page.process(interaction, &mut self.events, self.active_page_idx);

		self.painter.draw(&self.style, page, target)?;

		for (w, _rect) in page.iter_mut() {
			w.mark_clean();
		}

		Ok(())
	}
}
