use crate::{CharCell, CharCoord, Ordinal};
use derive_more::Display;
use getset::CopyGetters;
use std::fmt::{self, Debug, Formatter};

#[cfg(test)]
use pretty_assertions::assert_eq;

/// Information of a single line.
#[derive(Display, Clone, CopyGetters)]
#[display(fmt = "{src_text}")]
#[getset(get_copy = "pub")]
pub struct CharLine<'a> {
    /// Position of the line.
    pos: Ordinal,
    /// Total sizes of all lines before this line.
    offset: usize,
    /// Source text of the line.
    src_text: &'a str,
    /// List of characters.
    #[getset(skip)]
    char_list: Vec<CharCell>,
}

impl<'a> CharLine<'a> {
    /// Scan a line of text.
    pub(crate) fn scan_text(src_text: &'a str, ln_pred: usize, offset: usize) -> Self {
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
        CharLine {
            pos,
            offset,
            src_text,
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

impl<'a> Debug for CharLine<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let CharLine { pos, src_text, .. } = self;
        write!(f, "CharLine at {pos} of {src_text:?}")
    }
}

#[test]
fn test_char_offset() {
    let src_text = "I Love ❤️ Rust 🦀!";
    let char_line = CharLine::scan_text(src_text, 0, 0);
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
