use super::{PartiallyClonedCharGrid, PartiallyClonedCharGridBuilder};
use crate::{CharCell, CharOrEol, EndOfLine};

impl PartiallyClonedCharGrid {
    /// Create an empty builder.
    pub fn builder() -> PartiallyClonedCharGridBuilder {
        let PartiallyClonedCharGrid {
            char_count,
            text,
            char_list,
            line_list,
        } = Default::default();
        PartiallyClonedCharGridBuilder {
            char_count,
            text,
            char_list,
            line_list,
            current_ln_start: char_count,
        }
    }
}

impl PartiallyClonedCharGridBuilder {
    pub fn build(mut self) -> Option<PartiallyClonedCharGrid> {
        let last_char = self.char_list.last()?;
        let eof = CharCell {
            coord: last_char.coord().advance_column(1),
            pos: last_char.pos().advance_by(1),
            offset_from_ln_start: self
                .line_list
                .last()
                .map(|(line, _)| line.size())
                .unwrap_or(0), // NOTE: this could be incorrect
            offset_from_doc_start: self.text.len(), // NOTE: this could be incorrect
            value: CharOrEol::EndOfLine(EndOfLine::EOF),
        };
        self.push(eof);
        let PartiallyClonedCharGridBuilder {
            char_count,
            text,
            char_list,
            line_list,
            ..
        } = self;
        Some(PartiallyClonedCharGrid {
            char_count,
            text,
            char_list,
            line_list,
        })
    }
}
