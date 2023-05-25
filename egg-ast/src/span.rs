use crate::LnCol;

/// Range of source code correspond to a node in the syntax tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span(pub LnCol, pub LnCol);
