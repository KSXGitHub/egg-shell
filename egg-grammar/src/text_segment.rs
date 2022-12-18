use crate::{CharCell, CharCoord, Ordinal};
use getset::CopyGetters;
use std::fmt::{self, Debug, Display, Formatter};

#[cfg(test)]
use pretty_assertions::assert_eq;

/// Information of a single line.
#[derive(Clone, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct TextSegment {
    /// Position of the line.
    pos: Ordinal,
    /// Total sizes of all lines before this line.
    offset: usize,
    /// List of characters.
    #[getset(skip)]
    char_list: Vec<CharCell>,
}

impl TextSegment {
    /// Scan a line of text.
    pub(crate) fn scan_text(src_text: &str, ln_pred: usize, offset: usize) -> Self {
        let pos = Ordinal::from_pred_count(ln_pred);
        let mut offset_from_ln_start = 0;
        let mut char_list = Vec::new();
        for (col_pred, value) in src_text.chars().enumerate() {
            char_list.push(CharCell {
                coord: CharCoord::from_pred_counts(ln_pred, col_pred),
                offset_from_doc_start: offset + offset_from_ln_start,
                offset_from_ln_start,
                value,
            });
            offset_from_ln_start += value.len_utf8();
        }
        TextSegment {
            pos,
            offset,
            char_list,
        }
    }

    /// Number of characters.
    pub fn char_count(&self) -> usize {
        self.char_list.len()
    }

    /// Iterate over all character cells in the line.
    pub fn char_cells(&self) -> impl Iterator<Item = &CharCell> {
        self.char_list.iter()
    }
}

impl Display for TextSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for char_cell in self.char_cells() {
            write!(f, "{char_cell}")?;
        }
        Ok(())
    }
}

impl Debug for TextSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pos = self.pos;
        let text = self.to_string();
        write!(f, "CharLine at {pos} of {text:?}")
    }
}

#[test]
fn test_char_offset() {
    let src_text = "I Love ❤️ Rust 🦀!";
    let char_line = TextSegment::scan_text(src_text, 0, 0);
    let mut received = Vec::new();
    for char_cell in char_line.char_cells().copied() {
        dbg!(char_cell);
        let offset = char_cell.offset_from_ln_start();
        dbg!(offset);
        let before = &src_text[..offset];
        dbg!(before);
        let after = &src_text[offset..];
        dbg!(after);
        received.push((before, after));
    }
    dbg!(&received);
    let expected = [
        ("", "I Love ❤️ Rust 🦀!"),
        ("I", " Love ❤️ Rust 🦀!"),
        ("I ", "Love ❤️ Rust 🦀!"),
        ("I L", "ove ❤️ Rust 🦀!"),
        ("I Lo", "ve ❤️ Rust 🦀!"),
        ("I Lov", "e ❤️ Rust 🦀!"),
        ("I Love", " ❤️ Rust 🦀!"),
        ("I Love ", "❤️ Rust 🦀!"),
        ("I Love ❤", "\u{fe0f} Rust 🦀!"),
        ("I Love ❤️", " Rust 🦀!"),
        ("I Love ❤️ ", "Rust 🦀!"),
        ("I Love ❤️ R", "ust 🦀!"),
        ("I Love ❤️ Ru", "st 🦀!"),
        ("I Love ❤️ Rus", "t 🦀!"),
        ("I Love ❤️ Rust", " 🦀!"),
        ("I Love ❤️ Rust ", "🦀!"),
        ("I Love ❤️ Rust 🦀", "!"),
    ];
    assert_eq!(received, expected);
}
