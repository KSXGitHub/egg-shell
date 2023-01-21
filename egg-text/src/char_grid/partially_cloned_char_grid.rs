use crate::{CharCell, CharOrEol};
use getset::{CopyGetters, Getters};

#[derive(Debug, Clone, Copy)]
pub struct PartiallyClonedCharGridLineDef {
    pub start_index: usize,
    pub char_count: usize,
}

#[derive(Default, Clone, CopyGetters, Getters)]
pub struct PartiallyClonedCharGrid {
    #[getset(get_copy = "pub")]
    char_count: usize,
    #[getset(get = "pub")]
    text: String,
    #[getset(get = "pub")]
    char_list: Vec<CharCell<CharOrEol>>, // TODO: reduce memory cost by storing only big characters.
    #[getset(get = "pub")]
    line_list: Vec<PartiallyClonedCharGridLineDef>,
}

impl PartiallyClonedCharGrid {
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
