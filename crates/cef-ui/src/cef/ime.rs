use crate::{Color, Range};
use bindings::{cef_composition_underline_style_t, cef_composition_underline_t};
use std::ffi::c_int;

/// Composition underline style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositionUnderlineStyle {
    Solid,
    Dot,
    Dash,
    None
}

impl From<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn from(value: cef_composition_underline_style_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn from(value: &cef_composition_underline_style_t) -> Self {
        match value {
            cef_composition_underline_style_t::CEF_CUS_SOLID => CompositionUnderlineStyle::Solid,
            cef_composition_underline_style_t::CEF_CUS_DOT => CompositionUnderlineStyle::Dot,
            cef_composition_underline_style_t::CEF_CUS_DASH => CompositionUnderlineStyle::Dash,
            cef_composition_underline_style_t::CEF_CUS_NONE => CompositionUnderlineStyle::None
        }
    }
}

impl From<CompositionUnderlineStyle> for cef_composition_underline_style_t {
    fn from(value: CompositionUnderlineStyle) -> Self {
        cef_composition_underline_style_t::from(&value)
    }
}

impl From<&CompositionUnderlineStyle> for cef_composition_underline_style_t {
    fn from(value: &CompositionUnderlineStyle) -> Self {
        match value {
            CompositionUnderlineStyle::Solid => cef_composition_underline_style_t::CEF_CUS_SOLID,
            CompositionUnderlineStyle::Dot => cef_composition_underline_style_t::CEF_CUS_DOT,
            CompositionUnderlineStyle::Dash => cef_composition_underline_style_t::CEF_CUS_DASH,
            CompositionUnderlineStyle::None => cef_composition_underline_style_t::CEF_CUS_NONE
        }
    }
}

/// Structure representing IME composition underline information. This is a thin
/// wrapper around Blink's WebCompositionUnderline class and should be kept in
/// sync with that.
pub struct CompositionUnderline {
    /// Underline character range.
    range: Range,

    /// Text color.
    color: Color,

    /// Background color.
    background_color: Color,

    /// Set to true (1) for thick underline.
    thick: bool,

    /// Style.
    style: CompositionUnderlineStyle
}

impl From<cef_composition_underline_t> for CompositionUnderline {
    fn from(value: cef_composition_underline_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_composition_underline_t> for CompositionUnderline {
    fn from(value: &cef_composition_underline_t) -> Self {
        Self {
            range:            value.range.into(),
            color:            value.color.into(),
            background_color: value.background_color.into(),
            thick:            value.thick != 0,
            style:            value.style.into()
        }
    }
}

impl From<CompositionUnderline> for cef_composition_underline_t {
    fn from(value: CompositionUnderline) -> Self {
        Self::from(&value)
    }
}

impl From<&CompositionUnderline> for cef_composition_underline_t {
    fn from(value: &CompositionUnderline) -> Self {
        cef_composition_underline_t {
            range:            value.range.into(),
            color:            value.color.into(),
            background_color: value.background_color.into(),
            thick:            value.thick as c_int,
            style:            value.style.into()
        }
    }
}
