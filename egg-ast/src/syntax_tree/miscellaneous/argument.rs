use crate::{Expression, Identifier, SimplePath, Span};

pub type FunctionArgumentList = ArgumentList<Expression>;
pub type FunctionArgument = Argument<Expression>;
pub type FunctionArgumentBody = ArgumentBody<Expression>;
pub type FunctionNamedArgument = NamedArgument<Expression>;

pub type MetaArgumentList = ArgumentList<MetaArgumentValue>;
pub type MetaArgument = Argument<MetaArgumentValue>;
pub type MetaArgumentBody = ArgumentBody<MetaArgumentValue>;
pub type MetaNamedArgument = NamedArgument<MetaArgumentValue>;

#[derive(Debug)]
pub struct ArgumentList<Value> {
    pub span: Span,
    pub body: Box<[Argument<Value>]>,
}

#[derive(Debug)]
pub struct Argument<Value> {
    pub span: Span,
    pub body: ArgumentBody<Value>,
}

#[derive(Debug)]
pub enum ArgumentBody<Value> {
    Named(NamedArgument<Value>),
    Positional(Value),
}

#[derive(Debug)]
pub struct NamedArgument<Value> {
    pub span: Span,
    pub key: Identifier,
    pub value: Value,
}

#[derive(Debug)]
pub enum MetaArgumentValue {
    Expression(Expression),
    Path(SimplePath),
}
