use crate::token::ParseMiddleToken;
use split_first_char::split_first_char;

/// Token for bracket characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BracketToken {
    pub direction: BracketDirection,
    pub shape: BracketShape,
}

/// Open bracket or close bracket?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketDirection {
    /// When the character is one of `(`, `[`, `{`.
    Open,
    /// When the character is one of `)`, `]`, `}`.
    Close,
}

/// Round bracket, square bracket, or curly bracket?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BracketShape {
    /// Either `(` or `)`.
    Round,
    /// Either `[` or `]`.
    Square,
    /// Either `{` or `}`.
    Curly,
}

impl BracketToken {
    /// Infer a [`BracketToken`] from a [character](char).
    pub const fn from_char(char: char) -> Option<Self> {
        use BracketDirection::*;
        use BracketShape::*;
        let (direction, shape) = match char {
            '(' => (Open, Round),
            ')' => (Close, Round),
            '[' => (Open, Square),
            ']' => (Close, Square),
            '{' => (Open, Curly),
            '}' => (Close, Curly),
            _ => return None,
        };
        Some(BracketToken { direction, shape })
    }
}

impl<'a> ParseMiddleToken<&'a str> for BracketToken {
    fn parse(input: &'a str) -> Option<(Self, &'a str)> {
        let (first, rest) = split_first_char(input)?;
        let token = BracketToken::from_char(first)?;
        Some((token, rest))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse() {
        macro_rules! test_positive {
            ($input:literal -> $direction:ident $shape:ident $rest:literal) => {
                assert_eq!(
                    BracketToken::parse($input),
                    Some((
                        BracketToken {
                            direction: BracketDirection::$direction,
                            shape: BracketShape::$shape
                        },
                        $rest,
                    )),
                )
            };
        }

        macro_rules! test_negative {
            ($input:literal) => {
                assert_eq!(BracketToken::parse($input), None)
            };
        }

        test_positive!("(abc)" -> Open Round "abc)");
        test_positive!("), (" -> Close Round ", (");
        test_positive!("[abc]" -> Open Square "abc]");
        test_positive!("], [" -> Close Square ", [");
        test_positive!("{abc}" -> Open Curly "abc}");
        test_positive!("}, {" -> Close Curly ", {");

        test_negative!("");
        test_negative!("abc");
        test_negative!("<abc>");
    }
}
