use crate::{CharCell, CharOrEol, CharPos, EndOfLine, LnCol};
use getset::CopyGetters;

/// Information of a text slice.
#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct TextSliceDef {
    /// Total sizes of all lines before this line.
    pub(crate) offset: usize,
    /// Size of the text in the line.
    pub(crate) size: usize,
    /// Coordinate of the first character.
    pub(crate) first_char_coord: LnCol,
    /// Position of the first character.
    pub(crate) first_char_pos: CharPos,
    /// Number of characters in the slice.
    pub(crate) char_count: usize,
}

/// Prepare a text scanner.
#[must_use = "call the `run` method to scan the text"]
pub struct ScanText<'a> {
    /// Character list to append to.
    pub char_list: &'a mut Vec<CharCell<CharOrEol>>,
    /// Source text for reference.
    pub src_text: &'a str,
    /// Coordinate of the first character of the slice.
    pub first_char_coord: LnCol,
    /// Byte offset from the beginning of the source text
    /// to the first character of the slice.
    pub offset: usize,
    /// Type of end of line string.
    pub eol: EndOfLine,
}

impl<'a> ScanText<'a> {
    /// Scan a line of text and append characters into a `Vec`.
    pub fn run(self) -> TextSliceDef {
        let ScanText {
            char_list,
            src_text,
            first_char_coord,
            offset,
            eol,
        } = self;
        let first_char_pos = CharPos::from_pred_count(char_list.len());
        let initial_char_count = char_list.len();
        let mut offset_from_ln_start = 0;
        let mut coord = first_char_coord;
        let mut pos = first_char_pos;
        for value in src_text.chars() {
            char_list.push(CharCell {
                coord,
                pos,
                offset_from_doc_start: offset + offset_from_ln_start,
                offset_from_ln_start,
                value: CharOrEol::Char(value),
            });
            offset_from_ln_start += value.len_utf8();
            coord = coord.advance_column(1);
            pos = pos.advance_by(1);
        }
        let size = src_text.len();
        let char_count = char_list.len() - initial_char_count;
        char_list.push(CharCell {
            coord,
            pos,
            offset_from_doc_start: offset + offset_from_ln_start,
            offset_from_ln_start,
            value: CharOrEol::EndOfLine(eol),
        });
        TextSliceDef {
            offset,
            first_char_coord,
            first_char_pos,
            size,
            char_count,
        }
    }
}

#[cfg(test)]
mod test;
