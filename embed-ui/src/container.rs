use crate::widgets::Widget;

pub type WidgetId = usize;

#[derive(Debug)]
pub struct View<'a, const S: usize> {
	widgets:     [Option<Widget<'a>>; S],
	current_idx: usize,
}

impl<'a, const S: usize> View<'a, S> {
	pub const fn new() -> Self {
		Self {
			widgets:     [const { None }; S],
			current_idx: 0,
		}
	}

	pub const fn insert(&mut self, widget: Widget<'a>) -> WidgetId {
		self.current_idx += 1;
		self.widgets[self.current_idx] = Some(widget);
		self.current_idx
	}

	pub fn get(&self, index: WidgetId) -> Option<&Widget<'a>> {
		if index < self.current_idx {
			self.widgets.get(index)?.as_ref()
		} else {
			None
		}
	}

	pub fn get_mut(&mut self, index: WidgetId) -> Option<&mut Widget<'a>> {
		if index < self.current_idx {
			self.widgets.get_mut(index)?.as_mut()
		} else {
			None
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = &Widget<'a>> {
		self.widgets.iter().flatten()
	}
}
