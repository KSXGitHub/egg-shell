use super::{PartiallyClonedCharGrid, PartiallyClonedCharGridEol};
use crate::{CharCell, CharOrEol};

impl PartiallyClonedCharGrid {
    /// Add a character to the grid.
    pub fn push(&mut self, char: CharCell<CharOrEol>) {
        match *char.value() {
            CharOrEol::Char(char) => {
                self.text.push(char);
            }
            CharOrEol::EndOfLine(eol) => {
                let offset = self.text.len();
                let index = self.char_count;
                self.eol_list
                    .push(PartiallyClonedCharGridEol { offset, index });
                self.text.push_str(eol.as_ref());
            }
        }
        self.char_count += 1;
        self.char_list.push(char);
    }
}
