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

/// Prepare a text scanner.
#[must_use = "call the `run` method to scan the text"]
pub struct ScanText<'a> {
    /// Character list to append to.
    pub char_list: &'a mut Vec<CharCell>,
    /// Source text for reference.
    pub src_text: &'a str,
    /// Coordinate of the first character of the slice.
    pub first_char_coord: CharCoord,
    /// Byte offset from the beginning of the source text
    /// to the first character of the slice.
    pub offset: usize,
}

impl<'a> ScanText<'a> {
    /// Scan a line of text and append characters into a `Vec`.
    pub fn run(self) -> TextSliceDef {
        let ScanText {
            char_list,
            src_text,
            first_char_coord,
            offset,
        } = self;
        let mut offset_from_ln_start = 0;
        for (col_add, value) in src_text.chars().enumerate() {
            char_list.push(CharCell {
                coord: first_char_coord.advance_column(col_add),
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
    ScanText::run(ScanText {
        char_list: &mut char_list,
        src_text,
        first_char_coord: CharCoord::from_pred_counts(0, 0),
        offset: 0,
    });
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
