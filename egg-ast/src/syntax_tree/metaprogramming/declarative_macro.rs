use crate::{Attribute, MetaParameterList, OptionalIdentifier, Span};

#[derive(Debug)]
pub struct DeclarativeMacro {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub identifier: Option<OptionalIdentifier>,
    pub parameter: MetaParameterList,
    // TODO
}
