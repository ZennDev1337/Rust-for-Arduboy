//! This is the Module to interact in a save way with the Arduboy2 C++ library.
//!
//! All of the functions are safe wrapped inside the [Arduboy2] struct.
#[doc(hidden)]
pub mod arduboy2;
#[doc(hidden)]
pub mod binding;
#[doc(hidden)]
pub mod print;

pub mod sprites;
pub use arduboy2::{Arduboy2, Color, Point, Rect, FONT_HEIGHT, FONT_WIDTH, HEIGHT, WIDTH};
