use std::{
    fmt::{self, Debug, Display, Formatter},
    num::NonZeroUsize,
};

/// Ordinal numbers are number that represent position of an items.
///
/// The smallest ordinal is 1.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ordinal {
    /// Number of predecessors.
    ///
    /// This number is always equal to `ordinal - 1`.
    pred_count: usize,
}

impl Ordinal {
    /// Create an ordinal from a the number of predecessors (pred_count).
    ///
    /// Ordinal is always equal to `pred_count + 1`, so `from_pred_count(0)` would return a cardinal of 1.
    pub const fn from_pred_count(pred_count: usize) -> Self {
        Ordinal { pred_count }
    }

    /// Number of predecessors.
    ///
    /// This number is always equal to `ordinal - 1`, so `.pred_count()` on a cardinal of 1 would return 0.
    pub const fn pred_count(self) -> usize {
        self.pred_count
    }

    /// Get value of the ordinal as a number.
    pub const fn value(self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(self.pred_count + 1) }
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
        .map(Ordinal::from_pred_count)
        .map(|ordinal| ordinal.to_string())
        .collect::<Vec<_>>();
    let expected = ["1", "2", "3", "4", "5", "6"];
    assert_eq!(received, expected);
}
