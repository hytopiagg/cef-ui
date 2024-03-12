use bindings::cef_color_t;
use std::fmt::{Error, Formatter};

/// Represents a 32-bit ARGB color value.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Color(u32);

impl Color {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns a value with the specified component values in range 0.0 to 1.0.
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(
            ((alpha * 255.0) as u32).wrapping_shl(24)
                | ((red * 255.0) as u32).wrapping_shl(16)
                | ((green * 255.0) as u32).wrapping_shl(8)
                | ((blue * 255.0) as u32)
        )
    }

    pub fn to_raw(self) -> cef_color_t {
        self.0
    }

    pub fn r(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn g(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn b(&self) -> u8 {
        (self.0 >> 0) as u8
    }

    pub fn a(&self) -> u8 {
        (self.0 >> 24) as u8
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "#{:02X}{:02X}{:02X}{:02X}",
            self.r(),
            self.g(),
            self.b(),
            self.a()
        )
    }
}
