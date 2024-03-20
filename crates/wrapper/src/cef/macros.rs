use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FfiError {
    MissingFunctionPointer(&'static str)
}

impl Display for FfiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            FfiError::MissingFunctionPointer(name) => {
                write!(f, "Missing function pointer: {}", name)
            }
        }
    }
}

impl std::error::Error for FfiError {}

/// A macro to simplify the process of calling of calling CEF functions.
/// This will check if the function pointer is available and return an
/// error if it is not.
#[macro_export]
macro_rules! try_c {
    ($self:ident, $name:ident, $body:block) => {{
        use anyhow::anyhow;

        $self
            .0
            .$name
            .ok_or_else(|| anyhow!($crate::FfiError::MissingFunctionPointer(stringify!($name))))
            .and_then(|$name| unsafe { $body })
    }};
}
