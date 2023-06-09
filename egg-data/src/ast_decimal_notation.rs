use crate::AstDecimalDigitList;
use compute_float::compute_float;
use serde::{Deserialize, Serialize};

/// Syntactical representation of a decimal number.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AstDecimalNotation {
    /// Whether the number is negative.
    pub negative: bool,
    /// The integer part of the real number (before the decimal mark).
    pub integer: AstDecimalDigitList,
    /// The fractional part of the real number (after the decimal mark).
    pub fractional: AstDecimalDigitList,
    /// The exponent in the scientific notation.
    pub exponent: i32,
}

impl AstDecimalNotation {
    pub fn from_str<Integer, Fractional>(
        negative: bool,
        integer: Integer,
        fractional: Fractional,
        exponent: i32,
    ) -> Option<Self>
    where
        Integer: AsRef<str>,
        Fractional: AsRef<str>,
    {
        let integer = integer.as_ref().parse().ok()?;
        let fractional = fractional.as_ref().parse().ok()?;
        Some(AstDecimalNotation {
            negative,
            integer,
            fractional,
            exponent,
        })
    }

    /// Compute a floating-point number.
    fn to_float<Output: compute_float::Float>(&self) -> Option<Output> {
        let AstDecimalNotation {
            negative,
            integer,
            fractional,
            exponent,
        } = self;

        let exponent = *exponent - (fractional.len() as i32);

        let mut mantissa: u64 = 0;
        for digit in integer.iter().chain(fractional.iter()) {
            mantissa = mantissa
                .checked_mul(10)?
                .checked_add(digit.value() as u64)?;
        }

        compute_float(*negative, mantissa, exponent)
    }

    /// Compute a 32-bit floating-point number.
    pub fn to_f32(&self) -> Option<f32> {
        self.to_float()
    }

    /// Computer a 64-bit floating-point number.
    pub fn to_f64(&self) -> Option<f64> {
        self.to_float()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn to_float() {
        macro_rules! case {
            ($negative:literal, $integer:literal, $fractional:literal, $exponent:literal -> $output:literal) => {{
                eprintln!(
                    "CASE: {} {} {} {} -> {:?}",
                    $negative, $integer, $fractional, $exponent, $output,
                );
                let notation =
                    AstDecimalNotation::from_str($negative, $integer, $fractional, $exponent)
                        .expect("Parsing success");
                dbg!(&notation);
                let f32 = notation.to_f32();
                let f64 = notation.to_f64();
                assert_eq!(
                    (
                        f32,
                        f64,
                        f32.map(f32::is_sign_negative),
                        f64.map(f64::is_sign_negative),
                        format!("{f32:?} {f64:?}"),
                    ),
                    (
                        Some($output),
                        Some($output),
                        Some(($output as f32).is_sign_negative()),
                        Some(($output as f64).is_sign_negative()),
                        format!("{:?} {:?}", Some($output as f32), Some($output as f64)),
                    ),
                );
            }};
        }
        case!(false, "", "", 0 -> 0.0);
        case!(true, "", "", 0 -> -0.0);
        case!(false, "000", "", 0 -> 0.0);
        case!(false, "100", "", 0 -> 100.0);
        case!(false, "", "001", 0 -> 0.001);
        case!(false, "1", "", 32 -> 1e32);
        case!(true, "1", "", 32 -> -1e32);
        case!(false, "1234", "56", -2 -> 1234.56e-2);
        case!(false, "1234", "56", 2 -> 1234.56e2);
        case!(true, "1234", "56", -2 -> -1234.56e-2);
        case!(true, "1234", "56", 2 -> -1234.56e2);
    }
}
