use crate::{CharCell, CharCoord, Ordinal};

pub trait CharAt: LoadCharAt {
    type Error;
    fn char_at(&self, coord: CharCoord) -> Result<CharCell, <Self as CharAt>::Error>;
}

pub trait LoadCharAt {
    type Error;
    fn load_char_at(&mut self, coord: CharCoord) -> Result<CharCell, Self::Error>;
}

pub trait LineAt: LoadLineAt {
    type Error;
    fn line_at(&self, ln_num: Ordinal) -> Result<Self::Line, <Self as LineAt>::Error>;
}

pub trait LoadLineAt {
    type Line;
    type Error;
    fn load_line_at(&mut self, ln_num: Ordinal) -> Result<Self::Line, Self::Error>;
}

pub trait CharCount {
    fn char_count(&self) -> usize;
}

pub trait LineCount {
    fn line_count(&self) -> usize;
}
