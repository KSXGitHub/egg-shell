use std::fmt::Debug;
use strum::{AsRefStr, Display, IntoStaticStr};

/// String that ends a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, Display, IntoStaticStr)]
#[allow(clippy::upper_case_acronyms)]
pub enum EndOfLine {
    #[strum(serialize = "\n")]
    LF,
    #[strum(serialize = "\r\n")]
    CRLF,
    #[strum(serialize = "")]
    EOF,
}

impl EndOfLine {
    /// Get the length of the EOL sequence.
    #[allow(clippy::len_without_is_empty)] // is_empty on this type would be misleading
    pub fn len(self) -> usize {
        <&'static str>::from(self).len()
    }
}
