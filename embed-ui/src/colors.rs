use embedded_graphics::pixelcolor::{
	PixelColor,
	raw::{RawData, RawU24},
};

/// RGB666 color stored in ILI9488 wire format: 3 × u8, each channel
/// occupying bits [7:2] (i.e. 6-bit value already shifted left by 2).
/// sizeof = 3, align = 1 — safe to cast directly to &[u8].
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Rgb666_Packed {
	r: u8,
	g: u8,
	b: u8,
}

impl Rgb666_Packed {
	pub const BYTES_PER_PIXEL: usize = 3;

	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Self {
			r: r << 2,
			g: g << 2,
			b: b << 2,
		}
	}

	pub const fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
		Self {
			r: r & 0xFC,
			g: g & 0xFC,
			b: b & 0xFC,
		}
	}
}

impl PixelColor for Rgb666_Packed {
	type Raw = RawU24;
}

impl From<RawU24> for Rgb666_Packed {
	fn from(data: RawU24) -> Self {
		let v = data.into_inner();
		Self {
			r: ((v >> 16) & 0xFF) as u8,
			g: ((v >> 8) & 0xFF) as u8,
			b: (v & 0xFF) as u8,
		}
	}
}

impl From<Rgb666_Packed> for RawU24 {
	fn from(c: Rgb666_Packed) -> Self {
		RawU24::new(((c.r as u32) << 16) | ((c.g as u32) << 8) | c.b as u32)
	}
}
