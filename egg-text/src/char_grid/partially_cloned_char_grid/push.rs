use super::{PartiallyClonedCharGrid, PartiallyClonedCharGridEol};
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
                let start_offset = self.text.len();
                let start_index = self.char_count;
                self.eol_list.push(PartiallyClonedCharGridEol {
                    start_offset,
                    start_index,
                });
            }
        }
    }
}
