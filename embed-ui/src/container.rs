use crate::widgets::{Widget, WidgetKind};

pub type WidgetId = usize;

#[derive(Debug)]
pub struct View<'a, const S: usize> {
	widgets:   [Option<WidgetKind<'a>>; S],
	count:     WidgetId,
	focus_idx: WidgetId,
}

impl<'a, const S: usize> View<'a, S> {
	pub const fn new() -> Self {
		Self {
			widgets:   [const { None }; S],
			count:     0,
			focus_idx: 0,
		}
	}

	pub fn focus_next(&mut self) {
		self.set_focused((self.focus_idx + 1) % self.count);
	}

	pub fn focus_prev(&mut self) {
		let prev = if self.focus_idx == 0 {
			self.count - 1
		} else {
			self.focus_idx - 1
		};

		self.set_focused(prev);
	}

	pub fn insert(&mut self, widget: WidgetKind<'a>) -> WidgetId {
		let id = self.count;
		self.widgets[id] = Some(widget);
		self.count += 1;

		if self.count == 1 {
			self.set_focused(0); // first widget gets focus
		}

		id
	}

	pub fn get(&self, index: WidgetId) -> Option<&WidgetKind<'a>> {
		if index < self.count {
			self.widgets.get(index)?.as_ref()
		} else {
			None
		}
	}

	pub fn get_mut(&mut self, index: WidgetId) -> Option<&mut WidgetKind<'a>> {
		if index < self.count {
			self.widgets.get_mut(index)?.as_mut()
		} else {
			None
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = &WidgetKind<'a>> {
		self.widgets[..self.count].iter().flatten()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut WidgetKind<'a>> {
		self.widgets[..self.count].iter_mut().flatten()
	}

	fn set_focused(&mut self, new_idx: usize) {
		if let Some(w) = self.get_mut(self.focus_idx) {
			w.set_focus(false);
		}
		self.focus_idx = new_idx;
		if let Some(w) = self.get_mut(self.focus_idx) {
			w.set_focus(true);
		}
	}
}
