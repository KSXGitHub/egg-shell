use std::{
    fmt::{self, Debug, Display, Formatter},
    num::NonZeroUsize,
};

/// Ordinal numbers are number that represent position of an items.
///
/// The smallest ordinal is 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ordinal {
    /// Offset is always equal to ordinal - 1.
    offset: usize,
}

impl Ordinal {
    /// Create an ordinal from an offset.
    ///
    /// Ordinal is always equal to offset + 1, so `from_offset(0)` would return a cardinal of 1.
    pub const fn from_offset(offset: usize) -> Self {
        Ordinal { offset }
    }

    /// Get an offset from an ordinal.
    ///
    /// Offset is always equal to ordinal - 1, so `.offset()` on a cardinal of 1 would return 0.
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Get value of the ordinal as a number.
    pub const fn value(self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.offset + 1) }
    }
}

/// Display the value of the ordinal as a string.
impl Display for Ordinal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Display the value of the ordinal as a debug string.
impl Debug for Ordinal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Ordinal {}", self)
    }
}

#[test]
fn test_display() {
    let received = [0, 1, 2, 3, 4, 5]
        .into_iter()
        .map(Ordinal::from_offset)
        .map(|ordinal| ordinal.to_string())
        .collect::<Vec<_>>();
    let expected = ["1", "2", "3", "4", "5", "6"];
    assert_eq!(received, expected);
}
