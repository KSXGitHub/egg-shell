use derive_more::{AsRef, Constructor, Deref, From};
use egg_text::CharOrEol;

/// Check if a character is the end of a word.
pub trait IsEndOfWord: Copy {
    /// Check if a character is the end of a word.
    fn is_end_of_word(self, char: CharOrEol) -> bool;
}

/// Use a function to check the [end of word](IsEndOfWord).
#[derive(Debug, Clone, Copy, AsRef, Constructor, Deref, From)]
pub struct EndOfWordFn<Callback>(pub Callback)
where
    Callback: Fn(CharOrEol) -> bool + Copy;

impl<Callback> IsEndOfWord for EndOfWordFn<Callback>
where
    Callback: Fn(CharOrEol) -> bool + Copy,
{
    fn is_end_of_word(self, char: CharOrEol) -> bool {
        let EndOfWordFn(is_end_of_word) = self;
        is_end_of_word(char)
    }
}
