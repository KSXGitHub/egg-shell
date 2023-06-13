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
#[serde(try_from = "AstUintSerde", into = "AstUintSerde")]
pub struct AstUint(pub BigUint);

/// Intermediate type to serialize/deserialize [`AstUint`].
#[derive(Serialize, Deserialize)]
struct AstUintSerde(String);

impl From<AstUint> for AstUintSerde {
    fn from(AstUint(value): AstUint) -> Self {
        value.to_bytes_le().pipe(hex::encode).pipe(AstUintSerde)
    }
}

impl TryFrom<AstUintSerde> for AstUint {
    type Error = hex::FromHexError;

    fn try_from(AstUintSerde(value): AstUintSerde) -> Result<Self, Self::Error> {
        value
            .pipe(hex::decode)?
            .pipe_deref(BigUint::from_bytes_le)
            .pipe(AstUint)
            .pipe(Ok)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pipe_trait::Pipe;
    use pretty_assertions::assert_eq;
    use serde_json::{from_str as parse_json, json, to_string_pretty as json_str};
    use serde_yaml::{from_str as parse_yaml, to_string as yaml_str};

    #[test]
    fn ast_int_serde() {
        macro_rules! case {
            (number = $number:expr, json = $expected_json:expr, yaml = $expected_yaml:expr $(,)?) => {{
                let number: AstInt = $number.into();
                eprintln!("number = {number}");

                let received_json = json_str(&number).expect("Dump JSON");
                eprintln!("JSON:\n{received_json}\n");
                assert_eq!(received_json.trim(), $expected_json);

                let received_yaml = yaml_str(&number).expect("Dump YAML");
                eprintln!("YAML:\n{received_yaml}\n");
                assert_eq!(received_yaml.trim(), $expected_yaml);

                let from_json: AstInt = parse_json(&received_json).expect("Parse JSON");
                dbg!(&from_json);
                assert_eq!(from_json, number);

                let from_yaml: AstInt = parse_yaml(&received_yaml).expect("Parse YAML");
                dbg!(&from_yaml);
                assert_eq!(from_yaml, number);
            }};
        }

        case! {
            number = AstInt(BigInt::new(Sign::Plus, vec![0x5209BEA7, 0x1474952E, 0x7C6DFEA, 0x79A5B118])),
            json = json!({ "Positive": "a7be09522e957414eadfc60718b1a579" })
                .pipe_ref(json_str)
                .expect("Expected JSON"),
            yaml = "!Positive a7be09522e957414eadfc60718b1a579",
        };

        case! {
            number = AstInt(BigInt::new(Sign::Minus, vec![0x5209BEA7, 0x1474952E, 0x7C6DFEA, 0x79A5B118])),
            json = json!({ "Negative": "a7be09522e957414eadfc60718b1a579" })
                .pipe_ref(json_str)
                .expect("Expected JSON"),
            yaml = "!Negative a7be09522e957414eadfc60718b1a579",
        };

        case! {
            number = AstInt(BigInt::zero()),
            json = "\"Zero\"",
            yaml = "Zero",
        };
    }

    #[test]
    fn ast_uint_serde() {
        let number: AstUint = vec![0x5209BEA7, 0x1474952E, 0x7C6DFEA, 0x79A5B118]
            .pipe(BigUint::new)
            .into();
        eprintln!("number = {number}");

        let expected_components = "a7be09522e957414eadfc60718b1a579";

        let received_json = json_str(&number).expect("Dump JSON");
        eprintln!("JSON:\n{received_json}\n");
        let expected_json = json_str(&expected_components).expect("Expected JSON");
        assert_eq!(received_json, expected_json);

        let received_yaml = yaml_str(&number).expect("Dump YAML");
        eprintln!("YAML:\n{received_yaml}\n");
        let expected_yaml = yaml_str(&expected_components).expect("Expected YAML");
        assert_eq!(received_yaml, expected_yaml);

        let from_json: AstUint = parse_json(&received_json).expect("Parse JSON");
        dbg!(&from_json);
        assert_eq!(from_json, number);

        let from_yaml: AstUint = parse_yaml(&received_yaml).expect("Parse YAML");
        dbg!(&from_yaml);
        assert_eq!(from_yaml, number);
    }
}
