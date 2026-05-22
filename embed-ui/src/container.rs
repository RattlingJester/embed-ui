use embedded_graphics::{
	prelude::{DrawTarget, Point, Size},
	primitives::Rectangle,
};

use crate::{
	Error,
	input::Interaction,
	style::Style,
	widgets::{Widget, WidgetKind},
};

pub type WidgetId = usize;

#[derive(Debug)]
pub struct Page<const WIDGET_COUNT: usize> {
	widgets:   [Option<(WidgetKind, Rectangle)>; WIDGET_COUNT],
	count:     usize,
	focus_idx: WidgetId,
	layout:    Layout,
}

impl<const S: usize> Page<S> {
	pub const fn new(size: Size, wrap: bool, align: Align) -> Self {
		let layout = Layout::new(size, wrap, align);

		Self {
			widgets: [const { None }; S],
			count: 0,
			focus_idx: 0,
			layout,
		}
	}

	pub fn draw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		for (widget, rect) in self.iter_mut() {
			if widget.is_changed() {
				widget.draw(style, rect, interaction, target)?;
			}
		}

		Ok(())
	}

	pub fn redraw<D: DrawTarget>(
		&mut self,
		style: &Style<D::Color>,
		interaction: Option<Interaction>,
		target: &mut D,
	) -> Result<(), D::Error> {
		for (widget, rect) in self.iter_mut() {
			widget.draw(style, rect, interaction, target)?;
		}

		Ok(())
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

	pub fn get(&self, index: WidgetId) -> Option<&WidgetKind> {
		if index >= self.count {
			return None;
		}

		self.widgets
			.get(index)
			.and_then(|opt| opt.as_ref())
			.map(|(widget, _rect)| widget)
	}

	pub fn get_mut(&mut self, index: WidgetId) -> Option<&mut WidgetKind> {
		if index >= self.count {
			return None;
		}

		self.widgets
			.get_mut(index)
			.and_then(|opt| opt.as_mut())
			.map(|(widget, _rect)| widget)
	}

	pub fn iter(&self) -> impl Iterator<Item = &(WidgetKind, Rectangle)> {
		self.widgets[..self.count].iter().flatten()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (WidgetKind, Rectangle)> {
		self.widgets[..self.count].iter_mut().flatten()
	}

	pub fn insert(&mut self, widget: WidgetKind) -> Result<WidgetId, Error> {
		let rect = self.layout.next(widget.size())?;

		let id = self.count;
		self.widgets[id] = Some((widget, rect));
		self.count += 1;

		if self.count == 1 {
			self.set_focused(0); // first widget gets focus
		}

		Ok(id)
	}

	pub fn insert_next_row(&mut self, widget: WidgetKind) -> Result<WidgetId, Error> {
		let rect = self.layout.next_row(widget.size())?;

		let id = self.count;
		self.widgets[id] = Some((widget, rect));
		self.count += 1;

		Ok(id)
	}

	fn set_focused(&mut self, new_idx: usize) {
		if let Some(w) = self.get(new_idx)
			&& !w.is_focusable()
		{
			return;
		}

		if let Some(w) = self.get_mut(self.focus_idx) {
			w.set_focus(false);
		}

		self.focus_idx = new_idx;
		if let Some(w) = self.get_mut(self.focus_idx) {
			w.set_focus(true);
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub enum HorizontalAlign {
	#[default]
	Left,
	Center {
		columns: usize,
	},
	Right,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum VerticalAlign {
	#[default]
	Top,
	Center {
		rows: usize,
	},
	Bottom,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Align {
	pub horizontal: HorizontalAlign,
	pub vertical:   VerticalAlign,
}

/// Struct for managing placing of widgets in the [Ui]
///
/// ## Placement Rules
///
/// - Widgets are placed in rows, from left to right, from top to bottom
/// - Placement cannot happen outside of the bounds of the placer
#[derive(Debug, Default)]
pub struct Layout {
	/// Position of the top left corner
	pos:        Point,
	/// Current row
	row:        usize,
	/// Current column
	col:        usize,
	/// Height of current row
	row_height: usize,
	/// Bound of the placer
	bounds:     Size,
	/// Wrap to the next row if widget does not fit
	pub wrap:   bool,
	/// Widget alignment
	pub align:  Align,
}

impl Layout {
	pub const DEFAULT: Self = Self {
		pos:        Point { x: 0, y: 0 },
		row:        0,
		col:        0,
		row_height: 0,
		bounds:     Size {
			width:  0,
			height: 0,
		},
		wrap:       false,
		align:      Align {
			horizontal: HorizontalAlign::Left,
			vertical:   VerticalAlign::Top,
		},
	};

	pub const fn new(bounds: Size, wrap: bool, align: Align) -> Self {
		Self {
			row: 0,
			col: 0,
			pos: Point::zero(),
			row_height: 0,
			bounds,
			wrap,
			align,
		}
	}

	fn next(&mut self, size: Size) -> Result<Rectangle, Error> {
		if !self.check_bounds(size) {
			return Err(Error::NoSpaceLeft);
		}

		if self.wrap && (self.pos.x as u32 + size.width > self.bounds.width) {
			self.pos.x = 0;
			self.pos.y += self.row_height as i32;
			self.row_height = 0;
			self.row += 1;
			self.col = 0;
		}

		let (x_offset, advance_width) = match self.align.horizontal {
			HorizontalAlign::Left => (0, size.width as i32),
			HorizontalAlign::Right => (
				(self.bounds.width as i32 - size.width as i32) - self.pos.x,
				size.width as i32,
			),
			HorizontalAlign::Center { columns } => {
				let cols = columns as i32;

				let base_slot_width = self.bounds.width as i32 / cols;
				let remainder = self.bounds.width as i32 % cols;

				let current_slot_width = if (self.col as i32) < remainder {
					base_slot_width + 1
				} else {
					base_slot_width
				};

				let offset = (current_slot_width - size.width as i32) / 2;

				(offset, current_slot_width)
			}
		};

		let (y_offset, advance_height) = match self.align.vertical {
			VerticalAlign::Top => (0, size.height as i32),
			VerticalAlign::Bottom => (
				(self.bounds.height as i32 - size.height as i32) - self.pos.y,
				size.height as i32,
			),
			VerticalAlign::Center { rows } => {
				let rows_count = rows as i32;

				let base_row_height = self.bounds.height as i32 / rows_count;
				let remainder = self.bounds.height as i32 % rows_count;

				let current_row_height = if (self.row as i32) < remainder {
					base_row_height + 1
				} else {
					base_row_height
				};

				let offset = (current_row_height - size.height as i32) / 2;

				(offset, current_row_height)
			}
		};

		let widget_rect = Rectangle {
			top_left: Point {
				x: self.pos.x + x_offset,
				y: self.pos.y + y_offset,
			},
			size,
		};

		self.row_height = self.row_height.max(advance_height as usize);
		self.pos.x += advance_width;
		self.col += 1;

		Ok(widget_rect)
	}

	fn next_row(&mut self, size: Size) -> Result<Rectangle, Error> {
		if !self.check_bounds(size) {
			return Err(Error::NoSpaceLeft);
		}

		let next_y = self.pos.y + self.row_height as i32;

		self.pos.x = 0;
		self.pos.y = next_y;
		self.row_height = 0;
		self.row += 1;
		self.col = 0;

		Ok(Rectangle {
			top_left: Point {
				x: self.pos.x,
				y: self.pos.y,
			},
			size,
		})
	}

	const fn space_available(&self) -> Size {
		Size::new(
			self.bounds.width - self.pos.x as u32,
			self.bounds.height - self.pos.y as u32,
		)
	}

	const fn check_bounds(&self, pos: Size) -> bool {
		pos.width <= self.bounds.width && pos.height <= self.bounds.height
	}
}
