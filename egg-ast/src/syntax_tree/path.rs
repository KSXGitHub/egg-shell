use crate::{Identifier, Span};

#[derive(Debug)]
pub struct SimplePath {
    pub span: Span,
    pub root: Option<SimplePathRoot>,
    pub body: Box<[Identifier]>,
}

#[derive(Debug)]
pub enum SimplePathRoot {
    External(Box<str>),
    CurrentPackage,
    CurrentModule,
}
