use crate::{CharCell, CharCoord};
use getset::CopyGetters;

/// Information of a text slice.
#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct TextSliceDef {
    /// Total sizes of all lines before this line.
    offset: usize,
    /// Size of the text in the line.
    size: usize,
    /// Coordinate of the first character.
    first_char_coord: CharCoord,
    /// Number of characters in the slice.
    char_count: usize,
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
        let initial_char_count = char_list.len();
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
            first_char_coord,
            size: src_text.len(),
            char_count: char_list.len() - initial_char_count,
        }
    }
}
