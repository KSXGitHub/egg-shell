use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    span: Span,
    body: Vec<ProgramItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramItem {}
