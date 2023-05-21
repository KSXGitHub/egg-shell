/// Token for a line comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentToken<Content>(pub Content);

impl<'a> CommentToken<&'a str> {
    /// Parse an input text into a line comment.
    ///
    /// **Note:** `line` is assumed to not contain any EOL characters.
    pub fn parse(input: &'a str) -> Option<Self> {
        input.strip_prefix('#').map(CommentToken)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive() {
        macro_rules! case {
            ($input:literal -> $output:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(CommentToken::parse($input), Some(CommentToken($output)));
            }};
        }

        case!("#" -> "");
        case!("#this is a comment" -> "this is a comment");
        case!("# this is a comment" -> " this is a comment");
    }

    #[test]
    fn negative() {
        macro_rules! case {
            ($input:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(CommentToken::parse($input), None);
            }};
        }

        case!("");
        case!("this is not a comment");
    }
}
