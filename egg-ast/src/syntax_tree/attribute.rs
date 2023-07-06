use crate::{MetaArgumentList, SimplePath, Span};

#[derive(Debug)]
pub struct Attribute {
    pub span: Span,
    pub path: SimplePath,
    pub arguments: MetaArgumentList,
}
