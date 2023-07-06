use crate::{MetaArgumentList, SimplePath, Span};

#[derive(Debug)]
pub struct Attribute {
    pub span: Span,
    pub callee_path: SimplePath,
    pub arguments: MetaArgumentList,
}
