use derive_more::Display;

/// The bottom type. Created because the [`!`] type is not completely stable yet.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Never {}

impl Never {
    /// Return a [`!`].
    pub const fn resolve(self) -> ! {
        match self {}
    }
}
