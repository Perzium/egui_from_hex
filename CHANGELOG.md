# 0.1.0
Initial release

Added:
	- Trait HexColor
	- `from_hex()` and `from_hex_premultiplied()` functions
	- Support for prefixes such as `0x`, `#`, and `x`
	- Support for 3, 4, 6, and 8 byte hex values

# 0.1.1
Shame fix

Fixed:
	- Prefix handling, as it handled only `0x`, and overlooked `x`

# 0.1.2
Error handling instead of defaulting to Black

Added:
	- This CHANGELOG.md
	- Error handling for invalid hex codes
	- Examples to the README.md file

Modified:
	- `from_hex()` and `from_hex_premultiplied()` now return a `Result` instead of a `Color`. `.unwrap()` to get Color
	- Licensing changes; Now completely MIT, yes you can do ANYTHING =DDD