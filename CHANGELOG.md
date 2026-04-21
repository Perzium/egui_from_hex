# 0.2.2
Finalized the support for macros.

Note: Had a circular dependency. Fixed it!

# 0.2.1
Added support for macros.

Added:
	- A function which exports the `parse_hex_to_rgba()` function.
	- Feature flag: `macros`

# 0.2.0
Handling `from_u32()` and `from_u32_premultiplied`.
\+ Shame fixes.

Note: `from_u32()` and its siblings do **NOT** need to be unwrapped. 

Added:
	- `from_u32()` and `from_u32_premultiplied()` functions.

Modified:
	- Changed the order of versions in the changelog (latest versions are in the top now).

Fixed:
	- A typo in line 12 of the docs (Shame Fix).

# 0.1.2
Error handling instead of defaulting to Black

Added:
	- This CHANGELOG.md
	- Error handling for invalid hex codes
	- Examples to the README.md file

Modified:
	- `from_hex()` and `from_hex_premultiplied()` now return a `Result` instead of a `Color`. `.unwrap()` to get Color
	- Licensing changes; Now completely MIT, yes you can do ANYTHING =DDD

# 0.1.1
Shame fix

Fixed:
	- Prefix handling, as it handled only `0x`, and overlooked `x`

# 0.1.0
Initial release

Added:
	- Trait HexColor
	- `from_hex()` and `from_hex_premultiplied()` functions
	- Support for prefixes such as `0x`, `#`, and `x`
	- Support for 3, 4, 6, and 8 byte hex values