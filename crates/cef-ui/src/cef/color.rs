use bindings::cef_color_t;
use std::fmt::{Debug, Error, Formatter};

/// Represents a 32-bit ARGB color value.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    /// The color white.
    pub const WHITE: Self = Self {
        a: 255,
        r: 255,
        g: 255,
        b: 255
    };

    /// The color black.
    pub const BLACK: Self = Self {
        a: 255,
        r: 0,
        g: 0,
        b: 0
    };

    /// Returns a value with the specified component values in range 0.0 to 1.0.
    pub fn rgba(alpha: f32, red: f32, green: f32, blue: f32) -> Self {
        debug_assert!(alpha >= 0.0 && alpha <= 1.0);
        debug_assert!(red >= 0.0 && red <= 1.0);
        debug_assert!(green >= 0.0 && green <= 1.0);
        debug_assert!(blue >= 0.0 && blue <= 1.0);

        Self {
            a: (alpha * 255.0) as u8,
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
            b: (blue * 255.0) as u8
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl From<cef_color_t> for Color {
    fn from(value: cef_color_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_color_t> for Color {
    fn from(value: &cef_color_t) -> Self {
        Self {
            a: (value >> 24) as u8,
            r: (value >> 16) as u8,
            g: (value >> 8) as u8,
            b: (value >> 0) as u8
        }
    }
}

impl From<Color> for cef_color_t {
    fn from(value: Color) -> Self {
        Self::from(&value)
    }
}

impl From<&Color> for cef_color_t {
    fn from(value: &Color) -> Self {
        let a = (value.a as u32) << 24;
        let r = (value.r as u32) << 16;
        let g = (value.g as u32) << 8;
        let b = (value.b as u32) << 0;

        a | r | g | b
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "#{:02X}{:02X}{:02X}{:02X}",
            self.a, self.r, self.g, self.b
        )
    }
}
