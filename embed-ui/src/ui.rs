use embedded_graphics::prelude::{DrawTarget, PixelColor};
use heapless::spsc::Queue;

use crate::{
	container::{Page, WidgetId},
	input::{Event, Interaction},
	style::Style,
	widgets::{
		Widget, WidgetKind, button::Button, checkbox::Checkbox, label::Label, separator::Separator,
		textbox::Textbox,
	},
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct Ui<const WIDGET_COUNT: usize, const PAGE_COUNT: usize, C: PixelColor> {
	pages:           [Page<WIDGET_COUNT>; PAGE_COUNT],
	events:          Queue<Event, WIDGET_COUNT>,
	redraw_needed:   bool,
	active_page_idx: u8,
	interaction:     Option<Interaction>,
	pub style:       Style<C>,
}

impl<const WIDGET_COUNT: usize, const PAGE_COUNT: usize, C: PixelColor>
	Ui<WIDGET_COUNT, PAGE_COUNT, C>
{
	pub const fn new(pages: [Page<WIDGET_COUNT>; PAGE_COUNT], style: Style<C>) -> Self {
		Self {
			pages,
			events: Queue::new(),
			redraw_needed: false,
			active_page_idx: 0,
			style,
			interaction: None,
		}
	}

	pub fn drain_events(&mut self) -> Option<Event> {
		self.events.dequeue()
	}

	pub const fn switch_to_page(&mut self, idx: u8) -> bool {
		if idx < PAGE_COUNT as u8 {
			self.active_page_idx = idx;
			self.redraw_needed = true;
			true
		} else {
			false
		}
	}

	/// Switches to the next page, wrapping back to the first page if at the end.
	pub fn next_page(&mut self) {
		self.active_page_idx = (self.active_page_idx + 1) % PAGE_COUNT as u8;
		self.redraw_needed = true;
	}

	/// Switches to the previous page, wrapping to the last page if at the beginning.
	pub fn prev_page(&mut self) {
		self.active_page_idx = if self.active_page_idx == 0 {
			PAGE_COUNT as u8 - 1
		} else {
			self.active_page_idx - 1
		};

		self.redraw_needed = true;
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

	pub fn mark_clean(&mut self) {
		let page = self.current_page_mut();

		for (widget, _rect) in page.iter_mut() {
			widget.mark_clean();
		}
	}

	pub fn draw<D: DrawTarget<Color = C>>(
		&mut self,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		self.interaction = interaction;

		let active_page = &mut self.pages[self.active_page_idx as usize];

		if self.redraw_needed {
			target.clear(self.style.screen_bg)?;
			active_page.redraw(
				&self.style,
				self.interaction,
				&mut self.events,
				self.active_page_idx,
				target,
			)?;
			self.redraw_needed = false;
		} else {
			active_page.draw(
				&self.style,
				self.interaction,
				&mut self.events,
				self.active_page_idx,
				target,
			)?;
		}

		Ok(())
	}
}
