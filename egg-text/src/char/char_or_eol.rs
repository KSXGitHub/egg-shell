use super::{CharCell, EndOfLine};
use derive_more::{Display, From, TryInto};

/// Either a non-EOL character or and EOL sequence.
#[derive(Debug, Display, Clone, Copy, From, TryInto)]
pub enum CharOrEol {
    /// Non-EOL character.
    Char(char),
    /// End of line sequence.
    EndOfLine(EndOfLine),
}

impl TryFrom<CharCell<CharOrEol>> for CharCell<char> {
    type Error = &'static str;

    fn try_from(char_cell: CharCell<CharOrEol>) -> Result<Self, Self::Error> {
        let CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        } = char_cell;
        value.try_into().map(|value| CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        })
    }
}

impl TryFrom<CharCell<CharOrEol>> for CharCell<EndOfLine> {
    type Error = &'static str;

    fn try_from(char_cell: CharCell<CharOrEol>) -> Result<Self, Self::Error> {
        let CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        } = char_cell;
        value.try_into().map(|value| CharCell {
            coord,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        })
    }
}
