//! # egui_from_hex
//! 
//! EGUI from Hex is a simple, very lightweight and compatible crate to add to any of your projects.
//! 
//! It is made to work with older versions of EGUI, as well as improve the current `from_hex()` function.
//! 
//! 
//! ## Usage
//! 
//! The syntax is literally just how you use `Color32` normally.
//! 
//! Version 0.1.2 added error handling instead of defaulting to black if there were any errors.
//! Version 0.1.2 added error handling instead of defaulting to black if there were any errors.
//! If this inconveniences you, you can use the `unwrap()` function, or switch back to the previous version.
//! 
//! Using the `from_u32()` function and its siblings:
//! ```
//! // For u32
//! use egui_from_hex::HexColor;
//! let color = egui::Color32::from_u32(0xEDEBACFF);
//! ```
//! 
//! ```
//! // For u32 premultiplied
//! use egui_from_hex::HexColor;
//! let color = egui::Color32::from_u32_premultiplied(0xEDEBAC69);
//! ```
//! 
//! Note: `u32` functions do **NOT** need to be unwrapped.
//! 
//! Note: Some `u32` functions might require bitshifting (as some people might use RRGGBB, AARRGGBB, etc.)
//! but everything regarding that is explained within the function docs.
//! 
//! If you have any questions, feel free to contact me (Issues | Discussions).
//! 
//! ## Examples
//! 
//! Using the `from_hex()` function:
//! ```
//! use egui_from_hex::HexColor;
//! let color = egui::Color32::from_hex("#edebac").unwrap();
//! ```
//! 
//! Using the `from_hex()` function with error handling:
//! 1. Using the `match` statement
//! ```
//! use egui_from_hex::HexColor;
//! match egui::Color32::from_hex("#edebac")  {
//! 	Ok(color) => println!("Color: {:?}", color),
//! 	Err(e) => println!("Error: {:?}", e)
//! }
//! ```
//! 
//! 2. Using the `if let` statement
//! ```
//! use egui_from_hex::HexColor;
//! if let Ok(color) = egui::Color32::from_hex("#edebac") {
//! 	println!("Color: {:?}", color);
//! } else {
//! 	println!("Error... Yeah, you can't print an error here.");
//! }
//! ```
//! However, it's overkill to use the `match` or `if let` statements, you can just use the `unwrap()` function.
//! 
//! Note: Thank you for using my crate.

#![allow(clippy::tabs_in_doc_comments)]

use egui::Color32;
use std::fmt;

/// Types of errors while parsing a Hex string.
/// - InvalidLength: The characters entered is invalid.
/// - InvalidCharacter: The characters entered is not a valid Hex character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexParseError {
	InvalidLength,
	InvalidCharacter
}

/// Error handling for HexParseError
impl fmt::Display for HexParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidCharacter => write!(f, "[ERROR] Invalid character in HEX string"),
			Self::InvalidLength => write!(f, "[ERROR] Invalid HEX string length")
		}
	}
}

/// Implementing Error for HexParseError {}
impl std::error::Error for HexParseError {}

/// A basic trait; Zero-allocation Hex color parsing.
/// A basic trait; Zero-allocation Hex color parsing.
pub trait HexColor: Sized {

	/// ## Parses a u32 to a `Color32` using Straight Alpha.
	/// 
	/// Supports `RRGGBBAA` only.
	/// 
	/// If you're using a `u32` with `0xRRGGBB00`, use function as
	/// `from_u32_premultiplied(your_u32 | 0xFF)`
	/// 
	/// If you're using a `u32` with `0x00RRGBB`, use function as
	/// `from_u32_premultiplied((0xEDEBAC << 8) | 0xFF)`
	/// 
	/// ### Example
	/// ```
	/// use egui_from_hex::HexColor;
	/// let color = egui::Color32::from_u32(0xEDEBACFF);
	/// ```
	fn from_u32(color: u32) -> Self;
	
	/// ## Parses a u32 to a `Color32` using Premultiplied Alpha.
	/// 
	/// Supports `RRGGBBAA` only.
	/// 
	/// If you're using a `u32` with `0xRRGGBB00`, use function as
	/// `from_u32_premultiplied(your_u32 | 0xFF)`
	/// 
	/// If you're using a `u32` with `0x00RRGBB`, use function as
	/// `from_u32_premultiplied((0xEDEBAC << 8) | 0xFF)`
	/// 
	/// ### Example
	/// ```
	/// use egui_from_hex::HexColor;
	/// let color = egui::Color32::from_u32_premultiplied(0xEDEBAC69);
	/// ```
	fn from_u32_premultiplied(color: u32) -> Self;

	/// ## Parses a Hex string to a `Color32` using Straight Alpha.
	/// 
	/// Supports `RGB`, `RGBA`, `RRGGBB`, `RRGGBBAA`.
	/// 
	/// Unwrapping is usually fine as long as your HEX values are correct.
	/// Throws an error otherwise.
	/// 
	/// ### Example
	/// ```
	/// use egui_from_hex::HexColor;
	/// let color = egui::Color32::from_hex("#edebac").unwrap();
	/// ```
	/// 
	/// Note: You can use your preferred prefix, such as `#`, `0x`, `x`.
	/// 
	/// No prefixes are also supported.
	fn from_hex(hex: &str) -> Result<Self, HexParseError>;


	/// ## Parses a Hex string to a `Color32` using Premultiplied Alpha.
	/// 
	/// Supports `RGB`, `RGBA`, `RRGGBB`, `RRGGBBAA`.
	/// 
	/// Unwrapping is usually fine as long as your HEX values are correct.
	/// Throws an error otherwise.
	/// 
	/// ### Example
	/// ```
	/// use egui_from_hex::HexColor;
	/// let color = egui::Color32::from_hex("#edebac69").unwrap();
	/// ```
	/// 
	/// Note: You can use your preferred prefix, such as `#`, `0x`, `x`.
	/// 
	/// No prefixes are also supported.
	fn from_hex_premultiplied(hex: &str) -> Result<Self, HexParseError>;
}

/// Implementing HexColor for Color32
impl HexColor for Color32 {
	// Normal Hex Parsing
	#[inline]
	fn from_hex(hex: &str) -> Result<Self, HexParseError> {
		let (r, g, b, a) = parse_hex_to_rgba(hex)?;
		Ok(Self::from_rgba_unmultiplied(r, g, b, a))
	}

	// Premultiplied Hex Parsing
	#[inline]
	fn from_hex_premultiplied(hex: &str) -> Result<Self, HexParseError> {
		let (r, g, b, a) = parse_hex_to_rgba(hex)?;
		Ok(Self::from_rgba_premultiplied(r, g, b, a))
	}

	#[inline]
	fn from_u32(color: u32) -> Self {
		let (r, g, b, a) = (
			(color >> 24) as u8,
			(color >> 16) as u8,
			(color >> 8) as u8,
			(color) as u8,
		);

		Self::from_rgba_unmultiplied(r, g, b, a)
	}

	#[inline]
	fn from_u32_premultiplied(color: u32) -> Self {
		let (r, g, b, a) = (
			(color >> 24) as u8,
			(color >> 16) as u8,
			(color >> 8) as u8,
			color as u8,
		);

		Self::from_rgba_premultiplied(r, g, b, a)
	}
}

/// Function which actually does the work
fn parse_hex_to_rgba(hex: &str) -> Result<(u8, u8, u8, u8), HexParseError> {
	let bytes = hex.as_bytes();

	// Strip any prefixes
	let start = match bytes {
		[b'0', b'x', ..] => 2,
		[b'#', ..] | [b'x', ..] => 1,
		_ => 0,
	};

	let s = &bytes[start..];
	
	// ASCII to 0..15 helper
	fn hv(b: u8) -> Result<u8, HexParseError> {
		match b {
			b'0'..=b'9' => Ok(b - b'0'),
			b'a'..=b'f' => Ok(b - b'a' + 10),
			b'A'..=b'F' => Ok(b - b'A' + 10),
			_ => Err(HexParseError::InvalidCharacter),
		}
	}

	match s.len() {
		// 3 Byte Hex Values [RGB 1]
		3 => Ok((
			(hv(s[0])? << 4) | hv(s[0])?,
			(hv(s[1])? << 4) | hv(s[1])?,
			(hv(s[2])? << 4) | hv(s[2])?,
			255,
		)),
		// 4 Byte Hex Values [RGBA 1]
		4 => Ok((
			(hv(s[0])? << 4) | hv(s[0])?,
			(hv(s[1])? << 4) | hv(s[1])?,
			(hv(s[2])? << 4) | hv(s[2])?,
			(hv(s[3])? << 4) | hv(s[3])?,
		)),
		// 6 Byte Hex Values [RGB 2]
		6 => Ok((
			(hv(s[0])? << 4) | hv(s[1])?,
			(hv(s[2])? << 4) | hv(s[3])?,
			(hv(s[4])? << 4) | hv(s[5])?,
			255,
		)),
		// 8 Byte Hex Values [RGBA 2]
		8 => Ok((
			(hv(s[0])? << 4) | hv(s[1])?,
			(hv(s[2])? << 4) | hv(s[3])?,
			(hv(s[4])? << 4) | hv(s[5])?,
			(hv(s[6])? << 4) | hv(s[7])?,
		)),
		// Fallback
		_ => Err(HexParseError::InvalidLength),
	}
}