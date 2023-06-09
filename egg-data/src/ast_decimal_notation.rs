use crate::AstDecimalDigitList;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, str::FromStr};

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
    /// Create a decimal notation from strings of integer and fractional.
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
    fn to_float<Output: FromStr>(&self) -> Option<Output> {
        // No time to properly implement float parsing algorithm.
        // Let the Rust std library handle it.
        let AstDecimalNotation {
            negative,
            integer,
            fractional,
            exponent,
        } = self;
        let exp_len = 5;
        let mut string = String::with_capacity(
            '-'.len_utf8()
                + integer.len()
                + '.'.len_utf8()
                + fractional.len()
                + 'e'.len_utf8()
                + exp_len,
        );
        if *negative {
            string.push('-');
        };
        if integer.is_empty() {
            string.push_str("0.");
        } else {
            write!(string, "{integer}.").expect("Write integer to string");
        }
        if fractional.is_empty() {
            string.push('0');
        } else {
            write!(string, "{fractional}").expect("Write fractional to string");
        }
        if *exponent != 0 {
            write!(string, "e{exponent}").expect("Write exponent to string");
        }
        string.parse().ok()
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
