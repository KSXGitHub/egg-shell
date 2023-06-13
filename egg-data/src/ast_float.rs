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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::{from_str as parse_json, to_string_pretty as json_str};
    use serde_yaml::{from_str as parse_yaml, to_string as yaml_str};
    use text_block_macros::text_block;

    #[test]
    fn ast_float_serde() {
        macro_rules! case {
            (number = $number:expr, json = $expected_json:expr, yaml = $expected_yaml:expr, $(,)?) => {{
                let number = $number;

                let received_json = json_str(&number).expect("Dump JSON");
                eprintln!("JSON:\n{received_json}\n");
                assert_eq!(received_json.trim(), $expected_json);

                let received_yaml = yaml_str(&number).expect("Dump YAML");
                eprintln!("YAML:\n{received_yaml}\n");
                assert_eq!(received_yaml.trim(), $expected_yaml);

                let from_json: AstFloat = parse_json(&received_json).expect("Parse JSON");
                dbg!(&from_json);
                assert_eq!(from_json, number);

                let from_yaml: AstFloat = parse_yaml(&received_yaml).expect("Parse YAML");
                dbg!(&from_yaml);
                assert_eq!(from_yaml, number);
            }};
        }

        case! {
            number = AstFloat::Nan,
            json = "\"Nan\"",
            yaml = "Nan",
        };

        case! {
            number = AstFloat::NegativeInfinity,
            json = "\"NegativeInfinity\"",
            yaml = "NegativeInfinity",
        };

        case! {
            number = AstFloat::PositiveInfinity,
            json = "\"PositiveInfinity\"",
            yaml = "PositiveInfinity",
        };

        case! {
            number = AstFloat::Real(AstDecimalNotation {
                negative: false,
                integer: "3524".parse().unwrap(),
                fractional: "2353".parse().unwrap(),
                exponent: -3,
            }),
            json = text_block! {
                r#"{"#
                r#"  "Real": {"#
                r#"    "negative": false,"#
                r#"    "integer": "3524","#
                r#"    "fractional": "2353","#
                r#"    "exponent": -3"#
                r#"  }"#
                r#"}"#
            },
            yaml = text_block! {
                "!Real"
                "negative: false"
                "integer: '3524'"
                "fractional: '2353'"
                "exponent: -3"
            },
        };
    }
}
