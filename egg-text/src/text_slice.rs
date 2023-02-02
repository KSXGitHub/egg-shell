use crate::{CharCell, CharOrEol, CharPos, EndOfLine, LnCol};
use getset::CopyGetters;

/// Beginning or end of a text slice.
#[derive(Debug, Clone, Copy, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct TextSliceSep {
    /// Total sizes of all characters before this seperator.
    pub(crate) offset: usize,
    /// Position of the seperator.
    pub(crate) index: CharPos,
}

impl TextSliceSep {
    /// Seperator at the beginning of a text.
    pub const ZERO: Self = TextSliceSep {
        offset: 0,
        index: CharPos::from_pred_count(0),
    };
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
    pub fn run(self) -> (TextSliceSep, TextSliceSep) {
        let ScanText {
            char_list,
            src_text,
            first_char_coord,
            offset,
            eol,
        } = self;
        let first_char_pos = CharPos::from_pred_count(char_list.len());
        let start = TextSliceSep {
            offset,
            index: first_char_pos,
        };
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
        // char_list.push(CharCell {
        //     coord,
        //     pos,
        //     offset_from_doc_start: offset + offset_from_ln_start,
        //     offset_from_ln_start,
        //     value: CharOrEol::EndOfLine(eol),
        // }); // TODO: should this line be removed?
        let end = TextSliceSep {
            offset: offset + offset_from_ln_start,
            index: CharPos::from_pred_count(char_list.len()),
        };
        (start, end)
    }
}

#[cfg(test)]
mod test;
