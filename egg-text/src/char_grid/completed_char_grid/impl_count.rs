use crate::{CharCount, CompletedCharGrid, LineCount};

impl CharCount for CompletedCharGrid {
    fn char_count(&self) -> usize {
        let non_eol = self.char_list().len();
        let eol = self.eol_list().len();
        non_eol + eol
    }
}

impl LineCount for CompletedCharGrid {
    fn line_count(&self) -> usize {
        self.eol_list.len()
    }
}
