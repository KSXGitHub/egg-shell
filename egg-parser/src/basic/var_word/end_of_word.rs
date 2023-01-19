use egg_text::CharOrEol;

/// Check if a character is the end of a word.
pub trait IsEndOfWord: Copy {
    /// Check if a character is the end of a word.
    fn is_end_of_word(self, char: CharOrEol) -> bool;
}
