# egui_from_hex

EGUI from Hex is a simple, very lightweight and compatible crate to add to any of your projects.
It is made to work with older versions of EGUI, as well as improve the current `from_hex()` function.


## Usage
The syntax is literally just how you use `Color32` normally.

Version 0.1.2 added error handling instead of defaulting to black there were any errors.
If this inconveniences you, you can use the `unwrap()` function, or switch back to the previous version.

## Examples

Using the `from_hex()` function:
```rust
use egui_from_hex::HexColor;
let color = egui::Color32::from_hex("#edebac").unwrap();
```

Using the `from_hex()` function with error handling:
1. Using the `match` statement
```rust
use egui_from_hex::HexColor;
match egui::Color32::from_hex("#edebac")  {
	Ok(color) => println!("Color: {:?}", color),
	Err(e) => println!("Error: {}", e)
}
```

2. Using the `if let` statement
```rust
use egui_from_hex::HexColor;
if let Ok(color) = egui::Color32::from_hex("#edebac") {
	println!("Color: {:?}", color);
} else {
	println!("Error: {}", e);
}
```
However, it's overkill to use the `match` or `if let` statements, you can just use the `unwrap()` function.

Note: Thank you for using my crate.