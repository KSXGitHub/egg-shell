use super::{CharCell, EndOfLine};
use derive_more::{Display, From, TryInto};

/// Either a non-EOL character or and EOL sequence.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, From, TryInto)]
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
            pos,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        } = char_cell;
        value.try_into().map(|value| CharCell {
            coord,
            pos,
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
            pos,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        } = char_cell;
        value.try_into().map(|value| CharCell {
            coord,
            pos,
            offset_from_ln_start,
            offset_from_doc_start,
            value,
        })
    }
}

impl PartialEq<str> for CharOrEol {
    fn eq(&self, other: &str) -> bool {
        let char = match *self {
            CharOrEol::Char(char) => char,
            CharOrEol::EndOfLine(eol) => return eol.as_ref() == other,
        };

        let mut chars = other.chars();
        chars.next() == Some(char) && chars.next().is_none()
    }
}
