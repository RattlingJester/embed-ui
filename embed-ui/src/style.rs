use embedded_graphics::{
	mono_font::{MonoFont, ascii::FONT_6X12},
	pixelcolor::{Rgb565, Rgb666},
	prelude::PixelColor,
};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct Style<C: PixelColor> {
	/// Screen/canvas fill colour used by [`Gui::clear_screen`].
	pub screen_bg: C,

	/// Default widget background fill.
	pub bg_color: C,

	/// Widget background when pressed / checked / active.
	pub active_color: C,

	/// Widget background when hovered.
	pub hover_color: C,

	/// Widget border / outline colour.
	pub border_color: C,

	/// Colour used for all rendered text.
	pub text_color: C,

	/// Colour used for focus border
	pub focus_color: C,

	/// Stroke width in pixels for widget outlines.
	pub border_width: u32,

	/// Font used for all widget labels.
	/// Any of the `embedded_graphics::mono_font::ascii::FONT_*` constants work.
	pub font: &'static MonoFont<'static>,
}

pub const DEFAULT_STYLE_565: Style<Rgb565> = Style {
	screen_bg:    Rgb565::new(1, 3, 2),    // #080E14
	bg_color:     Rgb565::new(2, 7, 5),    // #101C28
	active_color: Rgb565::new(0, 30, 22),  // #0079b5
	hover_color:  Rgb565::new(0, 20, 12),  // #005163
	border_color: Rgb565::new(0, 42, 24),  // #00A8C0
	text_color:   Rgb565::new(27, 60, 30), // #D8F0F4
	focus_color:  Rgb565::new(255, 0, 0),  // #FF0000
	border_width: 1,
	font:         &FONT_6X12,
};

pub const DEFAULT_STYLE_666: Style<Rgb666> = Style {
	screen_bg:    Rgb666::new(1, 3, 2),    // #080E14
	bg_color:     Rgb666::new(2, 7, 5),    // #101C28
	active_color: Rgb666::new(0, 30, 22),  // #0079b5
	hover_color:  Rgb666::new(0, 20, 12),  // #005163
	border_color: Rgb666::new(0, 42, 24),  // #00A8C0
	text_color:   Rgb666::new(27, 60, 30), // #D8F0F4
	focus_color:  Rgb666::new(255, 0, 0),  // #FF0000
	border_width: 1,
	font:         &FONT_6X12,
};
