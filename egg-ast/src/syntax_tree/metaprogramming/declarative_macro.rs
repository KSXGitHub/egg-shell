use crate::{Attribute, MetaParameterList, OptionalIdentifier, Span, VisibilityModifier};

#[derive(Debug)]
pub struct DeclarativeMacro {
    pub span: Span,
    pub attributes: Box<[Attribute]>,
    pub visibility: Option<VisibilityModifier>,
    pub identifier: Option<OptionalIdentifier>,
    pub parameter: MetaParameterList,
    // TODO
}
