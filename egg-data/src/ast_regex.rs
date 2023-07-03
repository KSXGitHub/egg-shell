use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, Into};
use pipe_trait::Pipe;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Parsed Regex object for the AST.
#[derive(
    Debug, Display, Clone, AsMut, AsRef, Deref, DerefMut, From, Into, Deserialize, Serialize,
)]
#[serde(try_from = "AstRegexSerde", into = "AstRegexSerde")]
pub struct AstRegex(pub Regex);

#[derive(Deserialize, Serialize)]
struct AstRegexSerde(String);

impl From<AstRegex> for AstRegexSerde {
    fn from(value: AstRegex) -> Self {
        value.to_string().pipe(AstRegexSerde)
    }
}

impl TryFrom<AstRegexSerde> for AstRegex {
    type Error = <Regex as FromStr>::Err;
    fn try_from(AstRegexSerde(value): AstRegexSerde) -> Result<Self, Self::Error> {
        value.parse().map(AstRegex)
    }
}
