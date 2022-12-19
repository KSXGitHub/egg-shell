use crate::{CharCell, CharCoord};
use getset::CopyGetters;

#[cfg(test)]
use pretty_assertions::assert_eq;

/// Information of a text slice.
#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct TextSliceDef {
    /// Total sizes of all lines before this line.
    offset: usize,
    /// Size of the text in the line.
    size: usize,
}

impl TextSliceDef {
    /// Scan a line of text and append characters into a `Vec`.
    pub(crate) fn scan_text(
        char_list: &mut Vec<CharCell>,
        src_text: &str,
        ln_pred: usize,
        col_pred: usize,
        offset: usize,
    ) -> Self {
        let mut offset_from_ln_start = 0;
        for (col_add, value) in src_text.chars().enumerate() {
            char_list.push(CharCell {
                coord: CharCoord::from_pred_counts(ln_pred, col_pred + col_add),
                offset_from_doc_start: offset + offset_from_ln_start,
                offset_from_ln_start,
                value,
            });
            offset_from_ln_start += value.len_utf8();
        }
        TextSliceDef {
            offset,
            size: src_text.len(),
        }
    }
}

#[test]
fn test_char_offset() {
    let src_text = "I Love ❤️ Rust 🦀!";
    let mut char_list = Vec::new();
    TextSliceDef::scan_text(&mut char_list, src_text, 0, 0, 0);
    let mut received = Vec::new();
    for char_cell in char_list.iter().copied() {
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
