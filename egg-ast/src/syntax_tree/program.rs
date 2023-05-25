use crate::LnCol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    span: (LnCol, LnCol),
    body: Vec<ProgramItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramItem {}
