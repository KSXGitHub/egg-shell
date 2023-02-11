use super::PartiallyClonedCharGridBuilder;
use crate::{CharCell, CharOrEol, EndOfLine, TextSliceDef};

impl PartiallyClonedCharGridBuilder {
    /// Create a line.
    fn make_line(&mut self, eol: EndOfLine) {
        let line_first_char = self.char_list[self.current_ln_start];
        let line = TextSliceDef {
            offset: line_first_char.offset_from_doc_start(),
            size: self.text.len(),
            first_char_coord: line_first_char.coord(),
            first_char_pos: line_first_char.pos(),
            char_count: self.char_count - self.current_ln_start,
        };
        self.line_list.push((line, eol));
        self.current_ln_start = self.char_count;
    }

    /// Add a character.
    pub fn push(&mut self, char: CharCell<CharOrEol>) {
        match *char.value() {
            CharOrEol::Char(char) => {
                self.text.push(char);
            }
            CharOrEol::EndOfLine(eol) => {
                self.make_line(eol);
                self.text.push_str(eol.as_ref());
            }
        }
        self.char_count += 1;
        self.char_list.push(char);
    }
}
