use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From};
use std::{
    fmt::{self, Debug, Display, Formatter},
    num::NonZeroUsize,
    ops::{Deref, DerefMut, Index, IndexMut},
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

    /// Advance the cardinal by a number of steps.
    pub const fn advance_by(mut self, steps: usize) -> Self {
        self.pred_count += steps;
        self
    }

    /// Try retreat the cardinal by a number of steps.
    ///
    /// Return `None` if overflow occurred.
    pub fn try_retreat_by(self, steps: usize) -> Option<Self> {
        self.pred_count
            .checked_sub(steps)
            .map(Ordinal::from_pred_count)
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

/// Wrapper that allows indexing by [`Ordinal`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From)]
pub struct OrdinalIndexed<Indexed>(pub Indexed);

impl<Indexed> Index<Ordinal> for OrdinalIndexed<Indexed>
where
    Indexed: Deref,
    Indexed::Target: Index<usize>,
{
    type Output = <Indexed::Target as Index<usize>>::Output;
    fn index(&self, ordinal: Ordinal) -> &Self::Output {
        self.index(ordinal.pred_count())
    }
}

impl<Indexed> IndexMut<Ordinal> for OrdinalIndexed<Indexed>
where
    Indexed: DerefMut,
    Indexed::Target: IndexMut<usize>,
{
    fn index_mut(&mut self, ordinal: Ordinal) -> &mut Self::Output {
        self.index_mut(ordinal.pred_count())
    }
}

impl<Indexed> Index<usize> for OrdinalIndexed<Indexed>
where
    Indexed: Deref,
    Indexed::Target: Index<usize>,
{
    type Output = <Indexed::Target as Index<usize>>::Output;
    fn index(&self, index: usize) -> &Self::Output {
        self.as_ref().index(index)
    }
}

impl<Indexed> IndexMut<usize> for OrdinalIndexed<Indexed>
where
    Indexed: DerefMut,
    Indexed::Target: IndexMut<usize>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_mut().index_mut(index)
    }
}

/// Wrap a type in [`OrdinalIndexed`].
pub trait AsOrdinalIndexed {
    /// Wrap `self` in [`OrdinalIndexed`].
    fn ordinal_indexed(&self) -> OrdinalIndexed<&'_ Self> {
        OrdinalIndexed(self)
    }

    /// Wrap `self` in [`OrdinalIndexed`].
    fn ordinal_indexed_mut(&mut self) -> OrdinalIndexed<&'_ mut Self> {
        OrdinalIndexed(self)
    }
}

impl<Indexed: Index<usize>> AsOrdinalIndexed for Indexed {}
