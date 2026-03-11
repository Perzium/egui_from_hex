//! # egui_from_hex
//! 
//! EGUI from Hex is a simple, very lightweight and compatible crate to add to any of your projects.
//! It is made to work with older versions of EGUI, as well as improve the current `from_hex()` function.
//! 
//! The syntax is literally just how you use `Color32` normally.
//! Note: Thank you for using my crate.

use egui::Color32;

/// A basic trait; Low-allocation Hex color parsing.
pub trait HexColor: Sized {
	/// ## Parses a Hex string to a `Color32` using Straight Alpha.
	/// Supports `RGB`, `RGBA`, `RRGGBB`, `RRGGBBAA`.
	/// 
	/// ### Example
	/// ```
	/// use egui_from_hex::HexColor;
	/// let color = egui::Color32::from_hex("#edebac");
	/// ```
	/// Note: You can use your preferred prefix, such as `#`, `0x`, `x`.
	/// No prefixes are also supported.
	fn from_hex(hex: &str) -> Self;
	
	/// ## Parses a Hex string to a `Color32` using Premultiplied Alpha.
	/// Supports `RGB`, `RGBA`, `RRGGBB`, `RRGGBBAA`.
	/// 
	/// ### Example
	/// ```
	/// use egui_from_hex::HexColor;
	/// let color = egui::Color32::from_hex("#edebac67");
	/// ```
	/// Note: You can use your preferred prefix, such as `#`, `0x`, `x`.
	/// No prefixes are also supported.
	fn from_hex_premultiplied(hex: &str) -> Self;
}

impl HexColor for Color32 {
	// Normal Hex Parsing
	#[inline]
	fn from_hex(hex: &str) -> Self {
		let (r, g, b, a) = parse_hex_to_rgba(hex);
		Self::from_rgba_unmultiplied(r, g, b, a)
	}

	// Premultiplied Hex Parsing
	#[inline]
	fn from_hex_premultiplied(hex: &str) -> Self {
		let (r, g, b, a) = parse_hex_to_rgba(hex);
		Self::from_rgba_premultiplied(r, g, b, a)
	}
}

fn parse_hex_to_rgba(hex: &str) -> (u8, u8, u8, u8) {
	let bytes = hex.as_bytes();

	// Strip any prefixes
	let start = match bytes {
		[b'0', b'x', ..] => 2,
		[b'#', ..] | [b'x', ..] => 1,
		_ => 0,
	};

	let s = &bytes[start..];
	
	// ASCII to 0..15 helper
	fn hv(b: u8) -> u8 {
		match b {
			b'0'..=b'9' => b - b'0',
			b'a'..=b'f' => b - b'a' + 10,
			b'A'..=b'F' => b - b'A' + 10,
			_ => 0,
		}
	}

	match s.len() {
		// 3 Byte Hex Values [RGB 1]
		3 => (
			(hv(s[0]) << 4) | hv(s[0]),
			(hv(s[1]) << 4) | hv(s[1]),
			(hv(s[2]) << 4) | hv(s[2]),
			255,
		),
		// 4 Byte Hex Values [RGBA 1]
		4 => (
			(hv(s[0]) << 4) | hv(s[0]),
			(hv(s[1]) << 4) | hv(s[1]),
			(hv(s[2]) << 4) | hv(s[2]),
			(hv(s[3]) << 4) | hv(s[3]),
		),
		// 6 Byte Hex Values [RGB 2]
		6 => (
			(hv(s[0]) << 4) | hv(s[1]),
			(hv(s[2]) << 4) | hv(s[3]),
			(hv(s[4]) << 4) | hv(s[5]),
			255,
		),
		// 8 Byte Hex Values [RGBA 2]
		8 => (
			(hv(s[0]) << 4) | hv(s[1]),
			(hv(s[2]) << 4) | hv(s[3]),
			(hv(s[4]) << 4) | hv(s[5]),
			(hv(s[6]) << 4) | hv(s[7]),
		),
		// Fallback
		_ => (0, 0, 0, 255),
	}
}