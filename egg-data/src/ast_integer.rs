use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, Into};
use num::{bigint::Sign, BigInt, BigUint, Zero};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};

/// Signed integer number for the AST.
#[derive(
    Debug,
    Display,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    From,
    Into,
    Serialize,
    Deserialize,
)]
#[serde(from = "AstIntSerde", into = "AstIntSerde")]
pub struct AstInt(pub BigInt);

/// Intermediate type to serialize/deserialize [`AstInt`].
#[derive(Deserialize, Serialize)]
enum AstIntSerde {
    Zero,
    Negative(AstUint),
    Positive(AstUint),
}

impl From<AstInt> for AstIntSerde {
    fn from(AstInt(value): AstInt) -> Self {
        let (sign, abs) = value.into_parts();
        match sign {
            Sign::NoSign => AstIntSerde::Zero,
            Sign::Minus => abs.pipe(AstUint).pipe(AstIntSerde::Negative),
            Sign::Plus => abs.pipe(AstUint).pipe(AstIntSerde::Positive),
        }
    }
}

impl From<AstIntSerde> for AstInt {
    fn from(value: AstIntSerde) -> Self {
        let (sign, abs) = match value {
            AstIntSerde::Zero => return BigInt::zero().pipe(AstInt),
            AstIntSerde::Negative(abs) => (Sign::Minus, abs.into()),
            AstIntSerde::Positive(abs) => (Sign::Plus, abs.into()),
        };
        BigInt::from_biguint(sign, abs).pipe(AstInt)
    }
}

/// Unsigned integer number for the AST.
#[derive(
    Debug,
    Display,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    From,
    Into,
    Serialize,
    Deserialize,
)]
#[serde(from = "AstUintSerde", into = "AstUintSerde")]
pub struct AstUint(pub BigUint);

/// Intermediate type to serialize/deserialize [`AstUint`].
#[derive(Serialize, Deserialize)]
struct AstUintSerde(Vec<u8>);

impl From<AstUint> for AstUintSerde {
    fn from(AstUint(value): AstUint) -> Self {
        value.to_bytes_le().pipe(AstUintSerde)
    }
}

impl From<AstUintSerde> for AstUint {
    fn from(AstUintSerde(value): AstUintSerde) -> Self {
        value.pipe_deref(BigUint::from_bytes_le).pipe(AstUint)
    }
}
