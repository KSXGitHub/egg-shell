use crate::AstDecimalNotation;
use derive_more::{From, TryInto};
use serde::{Deserialize, Serialize};

/// Syntactical representation of a floating-point number.
#[derive(Debug, Clone, PartialEq, Eq, From, TryInto, Deserialize, Serialize)]
pub enum AstFloat {
    Nan,
    NegativeInfinity,
    PositiveInfinity,
    Real(AstDecimalNotation),
}
