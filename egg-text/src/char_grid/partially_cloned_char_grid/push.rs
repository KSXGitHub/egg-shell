use super::{PartiallyClonedCharGrid, PartiallyClonedCharGridLineDef};
use crate::{CharCell, CharOrEol};

impl PartiallyClonedCharGrid {
    /// Add a character to the grid.
    pub fn push(&mut self, char: CharCell<CharOrEol>) {
        self.char_count += 1;
        self.char_list.push(char);
        match *char.value() {
            CharOrEol::Char(char) => {
                self.text.push(char);
            }
            CharOrEol::EndOfLine(eol) => {
                self.text.push_str(eol.as_ref());
                let start_index = self.char_count;
                let last_start_index = match self.line_list.last() {
                    Some(last_line) => last_line.start_index,
                    None => 0,
                };
                let char_count = self.char_count - last_start_index;
                self.line_list.push(PartiallyClonedCharGridLineDef {
                    start_index,
                    char_count,
                });
            }
        }
    }
}
