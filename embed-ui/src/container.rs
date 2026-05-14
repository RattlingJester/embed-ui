use crate::widgets::WidgetKind;

pub type WidgetId = usize;

#[derive(Debug)]
pub struct View<'a, const S: usize> {
	widgets:     [Option<WidgetKind<'a>>; S],
	current_idx: usize,
}

impl<'a, const S: usize> View<'a, S> {
	pub const fn new() -> Self {
		Self {
			widgets:     [const { None }; S],
			current_idx: 0,
		}
	}

	pub fn insert(&mut self, widget: WidgetKind<'a>) -> WidgetId {
		let id = self.current_idx;
		self.widgets[id] = Some(widget);
		self.current_idx += 1;
		id
	}

	pub fn get(&self, index: WidgetId) -> Option<&WidgetKind<'a>> {
		if index < self.current_idx {
			self.widgets.get(index)?.as_ref()
		} else {
			None
		}
	}

	pub fn get_mut(&mut self, index: WidgetId) -> Option<&mut WidgetKind<'a>> {
		if index < self.current_idx {
			self.widgets.get_mut(index)?.as_mut()
		} else {
			None
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = &WidgetKind<'a>> {
		self.widgets[..self.current_idx].iter().flatten()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut WidgetKind<'a>> {
		self.widgets[..self.current_idx].iter_mut().flatten()
	}
}
