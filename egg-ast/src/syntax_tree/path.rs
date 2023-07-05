use crate::{Identifier, Span};
use std::borrow::Cow;

#[derive(Debug)]
pub struct SimplePath {
    pub span: Span,
    pub root: Option<SimplePathRoot>,
    pub rest: Box<[Identifier]>,
}

#[derive(Debug)]
pub enum SimplePathRoot {
    CurrentModule,
    CurrentFile,
    External(Cow<'static, str>),
}
