use crate::{Identifier, OptionalIdentifier, Span, SyntaxTypeAnnotation};

#[derive(Debug)]
pub struct MetaParameterList {
    pub span: Span,
    pub main: Box<[MetaParameter]>,
}

#[derive(Debug)]
pub struct MetaParameter {
    pub span: Span,
    pub binding: MetaParameterBinding,
    pub syntax_type: SyntaxTypeAnnotation,
    // TODO
}

#[derive(Debug)]
pub enum MetaParameterBinding {
    Positional(OptionalIdentifier),
    Named(MetaParameterBindingRename),
}

#[derive(Debug)]
pub struct MetaParameterBindingRename {
    pub key: Identifier,
    pub target: OptionalIdentifier,
}
