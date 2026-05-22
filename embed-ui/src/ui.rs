use embedded_graphics::prelude::DrawTarget;

use crate::{container::Page, input::Interaction, style::Style};

pub struct Ui<const WIDGET_COUNT: usize, const PAGE_COUNT: usize, D: DrawTarget> {
	pages:           [Page<WIDGET_COUNT>; PAGE_COUNT],
	redraw_needed:   bool,
	active_page_idx: u8,
	pub style:       Style<D::Color>,
	pub interaction: Option<Interaction>,
}

impl<const WIDGET_COUNT: usize, const PAGE_COUNT: usize, D: DrawTarget>
	Ui<WIDGET_COUNT, PAGE_COUNT, D>
{
	pub const fn new(pages: [Page<WIDGET_COUNT>; PAGE_COUNT], style: Style<D::Color>) -> Self {
		Self {
			pages,
			redraw_needed: false,
			active_page_idx: 0,
			style,
			interaction: None,
		}
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

	pub fn get_page(&self, idx: usize) -> &Page<WIDGET_COUNT> {
		&self.pages[idx]
	}

	pub fn get_page_mut(&mut self, idx: usize) -> &mut Page<WIDGET_COUNT> {
		&mut self.pages[idx]
	}

	pub fn draw(&mut self, target: &mut D) -> Result<(), D::Error> {
		let active_page = &mut self.pages[self.active_page_idx as usize];

		if self.redraw_needed {
			target.clear(self.style.screen_bg)?;
			active_page.redraw(&self.style, self.interaction, target)?;
			self.redraw_needed = false;
		} else {
			active_page.draw(&self.style, self.interaction, target)?;
		}

		Ok(())
	}
}
