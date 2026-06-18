use embedded_graphics::{
	prelude::{Point, Size},
	primitives::Rectangle,
};
use heapless::spsc::Queue;

use crate::{
	Error,
	input::{Event, Interaction},
	widgets::{Widget, WidgetKind},
};

pub type WidgetId = usize;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct Page<const WIDGET_COUNT: usize> {
	pub(crate) widgets: [Option<(WidgetKind, Rectangle)>; WIDGET_COUNT],
	pub(crate) count:   usize,
	focus_idx:          WidgetId,
	layout:             Layout,
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

	/// Returns a bitmask of strips that contain at least one dirty widget.
	/// Bit N is set if strip N needs repainting.
	/// Pass STRIP_COUNT as a const generic to cap the bit width.
	pub fn dirty_strip_mask(&self, strip_h: usize, strip_count: usize) -> u32 {
		let mut mask = 0u32;
		for entry in self.widgets[..self.count].iter().flatten() {
			let (widget, rect) = entry;
			if widget.is_dirty() {
				let y0 = rect.top_left.y.max(0) as usize;
				let y1 = (y0 + rect.size.height as usize).saturating_sub(1);
				let first = y0 / strip_h;
				let last = (y1 / strip_h).min(strip_count - 1);
				for s in first..=last {
					mask |= 1 << s;
				}
			}
		}
		mask
	}

	pub fn process<const N: usize>(
		&mut self,
		interaction: Option<Interaction>,
		events: &mut Queue<Event, N>,
		page_idx: u8,
	) {
		for (widget_id, (widget, rect)) in
			self.widgets[..self.count].iter_mut().flatten().enumerate()
		{
			let is_hit = interaction
				.map(|i| rect.contains(i.point()))
				.unwrap_or(false);

			if is_hit || widget.is_pressed() {
				widget.interact(interaction);
			}

			match widget {
				WidgetKind::Button(b) if b.is_clicked() => unsafe {
					events.enqueue_unchecked(Event::ButtonClicked {
						page_idx,
						widget_id,
					})
				},
				WidgetKind::RadioButton(b) if b.is_clicked() => unsafe {
					events.enqueue_unchecked(Event::RadioButtonToggled {
						page_idx,
						widget_id,
					})
				},
				WidgetKind::Checkbox(c) if c.is_checked() => unsafe {
					events.enqueue_unchecked(Event::CheckboxToggled {
						page_idx,
						widget_id,
					})
				},
				_ => (),
			}
		}
	}

	pub fn focused(&self) -> WidgetId {
		self.focus_idx
	}

	pub fn focus_next(&mut self) {
		if self.count == 0 {
			return;
		}

		let start_idx = (self.focus_idx + 1) % self.count;
		let mut next_idx = start_idx;

		loop {
			if self.focus_set(next_idx) {
				return;
			}

			next_idx = (next_idx + 1) % self.count;

			if next_idx == start_idx {
				return;
			}
		}
	}

	pub fn focus_prev(&mut self) {
		if self.count == 0 {
			return;
		}

		let start_idx = if self.focus_idx == 0 {
			self.count - 1
		} else {
			self.focus_idx - 1
		};
		let mut prev_idx = start_idx;

		loop {
			if self.focus_set(prev_idx) {
				return;
			}

			prev_idx = if prev_idx == 0 {
				self.count - 1
			} else {
				prev_idx - 1
			};

			if prev_idx == start_idx {
				return;
			}
		}
	}

	/// Returns false if `new_idx` is not focusable
	pub fn focus_set(&mut self, new_idx: WidgetId) -> bool {
		if new_idx >= self.count {
			return false;
		}

		if let Some(w) = self.get(new_idx)
			&& !w.is_focusable()
		{
			return false;
		}

		if let Some(w) = self.get_mut(self.focus_idx) {
			w.set_focus(false);
		}

		self.focus_idx = new_idx;
		if let Some(w) = self.get_mut(self.focus_idx) {
			w.set_focus(true);
		}

		true
	}

	pub fn widget_interact(&mut self, widget_id: WidgetId, interaction: Interaction) -> bool {
		if let Some((w, _)) = &mut self.widgets[widget_id] {
			w.interact(Some(interaction));
		}

		false
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
			self.focus_set(0); // first widget gets focus
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
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Default, Clone, Copy)]
pub enum HorizontalAlign {
	#[default]
	Left,
	Center {
		columns: usize,
	},
	Right,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Default, Clone, Copy)]
pub enum VerticalAlign {
	#[default]
	Top,
	Center {
		rows: usize,
	},
	Bottom,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
///
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
	pub bounds: Size,
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

	const fn next_row(&mut self, size: Size) -> Result<Rectangle, Error> {
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

	const fn check_bounds(&self, pos: Size) -> bool {
		pos.width <= self.bounds.width && pos.height <= self.bounds.height
	}
}
