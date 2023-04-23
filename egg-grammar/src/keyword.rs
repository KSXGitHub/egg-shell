use strum::{AsRefStr, Display, EnumCount, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // essential std traits
#[derive(AsRefStr, Display, EnumCount, EnumIter, EnumString, EnumVariantNames, IntoStaticStr)] // essential strum traits
#[strum(use_phf, serialize_all = "lowercase")]
pub enum Keyword {
    /* Visibility keywords */
    Pub,

    /* Declaration keywords */
    Const,
    Enum,
    Fn,
    Impl,
    Let,
    Mod,
    Mut,
    Static,
    Struct,
    Trait,
    Type,
    Union,
    Where,
    With,

    /* Escape keywords */
    Break,
    Continue,
    Return,

    /* Literal keywords */
    True,
    False,
    Null,
    Inf,
    Nan,

    /* Imperative keywords */
    Case,
    Do,
    Else,
    Exec,
    For,
    If,
    Loop,
    Match,
    Then,
    While,

    /* Type keywords */
    Any,
    Never,
    Void,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,

    /* Operators */
    Await,
    From,
    In,
    Infer,
    Is,
    Not,
    Of,
    To,
    Yield,
}
