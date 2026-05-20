use embedded_graphics::{
	prelude::{DrawTarget, Point, Primitive, Size},
	primitives::{PrimitiveStyleBuilder, Rectangle},
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
	widgets:    [Option<(WidgetKind, Rectangle)>; WIDGET_COUNT],
	// widget_rects: [Option<Rectangle>; WIDGET_COUNT],
	count:      usize,
	focus_idx:  WidgetId,
	pub layout: Layout,
}

impl<const S: usize> Page<S> {
	pub const fn new(size: Size, wrap: bool, align: Align) -> Self {
		let layout = Layout::new(size, wrap, align);

		Self {
			widgets: [const { None }; S],
			// widget_rects: [const { None }; S],
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
				let bg = match interaction {
					Some(Interaction::Hover(p)) => {
						if rect.contains(p) {
							style.active_color
						} else {
							style.bg_color
						}
					}
					None => style.bg_color,
					_ => todo!(),
				};

				let prim_style = PrimitiveStyleBuilder::new()
					.stroke_color(style.border_color)
					.stroke_width(style.border_width)
					.fill_color(bg)
					.build();

				let styled_rect = rect.into_styled(prim_style);

				widget.draw(style, styled_rect, target)?;
			}
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
		let rect = self.allocate_space(widget.size())?;

		let id = self.count;
		self.widgets[id] = Some((widget, rect));
		self.count += 1;

		if self.count == 1 {
			self.set_focused(0); // first widget gets focus
		}

		Ok(id)
	}

	fn allocate_space(&mut self, size: Size) -> Result<Rectangle, Error> {
		let rect = self.layout.next(size)?;
		Ok(rect)
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

#[derive(Debug, Default, Clone, Copy)]
pub enum HorizontalAlign {
	Left,
	#[default]
	Center,
	Right,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum VerticalAlign {
	Top,
	#[default]
	Center,
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
			horizontal: HorizontalAlign::Center,
			vertical:   VerticalAlign::Center,
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

		let rem_width = self.bounds.width - self.pos.x as u32;
		let rem_height = self.bounds.height - self.pos.y as u32;

		let (x_offset, y_offset) = match (self.align.horizontal, self.align.vertical) {
			(HorizontalAlign::Left, VerticalAlign::Top) => (0, 0),
			(HorizontalAlign::Left, VerticalAlign::Center) => (0, (rem_height - size.height) / 2),
			(HorizontalAlign::Left, VerticalAlign::Bottom) => (0, rem_height - size.height),

			(HorizontalAlign::Center, VerticalAlign::Top) => ((rem_width - size.width) / 2, 0),
			(HorizontalAlign::Center, VerticalAlign::Center) => {
				((rem_width - size.width) / 2, (rem_height - size.height) / 2)
			}
			(HorizontalAlign::Center, VerticalAlign::Bottom) => {
				((rem_width - size.width) / 2, rem_height - size.height)
			}

			(HorizontalAlign::Right, VerticalAlign::Top) => (rem_width - size.width, 0),
			(HorizontalAlign::Right, VerticalAlign::Center) => {
				(rem_width - size.width, (rem_height - size.height) / 2)
			}
			(HorizontalAlign::Right, VerticalAlign::Bottom) => {
				(rem_width - size.width, rem_height - size.height)
			}
		};

		let widget_rect = Rectangle {
			top_left: Point {
				x: self.pos.x + x_offset as i32,
				y: self.pos.y + y_offset as i32,
			},
			size,
		};

		self.row_height = self.row_height.max(size.height as usize);
		self.pos.x += size.width as i32;
		self.col += 1;

		Ok(widget_rect)
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
