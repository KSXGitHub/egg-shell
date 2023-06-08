use crate::{AstInt, AstUint};
use num::{bigint::Sign, BigInt, BigRational, ToPrimitive};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};

/// Fraction number for the AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AstFraction {
    /// Numerator of the fraction.
    pub numerator: AstInt,
    /// Denominator of the fraction.
    pub denominator: AstUint,
}

impl AstFraction {
    /// Convert the fraction into an [`f32`].
    pub fn into_f32(self) -> Option<f32> {
        self.pipe(BigRational::from).to_f32()
    }

    /// Convert the fraction into an [`f64`].
    pub fn into_f64(self) -> Option<f64> {
        self.pipe(BigRational::from).to_f64()
    }
}

impl From<AstFraction> for BigRational {
    fn from(value: AstFraction) -> Self {
        let AstFraction {
            numerator,
            denominator,
        } = value;
        let denominator = BigInt::from_biguint(Sign::Plus, denominator.into());
        BigRational::new(numerator.into(), denominator)
    }
}
