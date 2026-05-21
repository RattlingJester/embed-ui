use embedded_graphics::prelude::DrawTarget;

use crate::{container::Page, input::Interaction, style::Style};

pub struct Ui<const WIDGET_COUNT: usize, const PAGE_COUNT: usize, D: DrawTarget> {
	pages:           [Page<WIDGET_COUNT>; PAGE_COUNT],
	pub style:       Style<D::Color>,
	pub interaction: Option<Interaction>,
}

impl<'a, const WIDGET_COUNT: usize, const PAGE_COUNT: usize, D: DrawTarget>
	Ui<WIDGET_COUNT, PAGE_COUNT, D>
{
	pub const fn new(pages: [Page<WIDGET_COUNT>; PAGE_COUNT], style: Style<D::Color>) -> Self {
		Self {
			pages,
			style,
			interaction: None,
		}
	}

	pub fn get_page(&'a self, idx: usize) -> &'a Page<WIDGET_COUNT> {
		&self.pages[idx]
	}

	pub fn get_page_mut(&'a mut self, idx: usize) -> &'a mut Page<WIDGET_COUNT> {
		&mut self.pages[idx]
	}

	pub fn draw(&mut self, target: &mut D) -> Result<(), D::Error> {
		for page in self.pages.iter_mut() {
			page.draw(&self.style, self.interaction, target)?;
		}

		Ok(())
	}
}
