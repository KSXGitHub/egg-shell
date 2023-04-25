use super::IndentChar;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator};
use egg_common_utils::split_first_char;
use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};

/// Token of indentation.
#[derive(Clone, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator)]
pub struct IndentToken(Vec<IndentChar>);

impl IndentToken {
    /// Parse a line of text into a pair of indentation and remaining string.
    ///
    /// **Notes:**
    /// * `line` is assumed to not contain any EOL characters.
    pub fn parse_line(mut line: &str) -> (Self, &'_ str) {
        let mut indent_char_list = Vec::with_capacity(line.len());
        while let Some((first, rest)) = split_first_char(line) {
            let Ok(indent) = first.try_into() else {
                break;
            };
            indent_char_list.push(indent);
            line = rest;
        }
        indent_char_list.shrink_to_fit();
        (IndentToken(indent_char_list), line)
    }
}

impl Debug for IndentToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IndentToken [")?;

        let consecutive_indents =
            self.iter()
                .map(|&indent| (indent, 1u32))
                .coalesce(|(a, ac), (b, bc)| {
                    if a == b {
                        Ok((a, ac + bc))
                    } else {
                        Err(((a, ac), (b, bc)))
                    }
                });

        for (indent, count) in consecutive_indents {
            let abbr = indent.abbr();
            if count > 1 {
                write!(f, "<{abbr}✕{count}>")?;
            } else {
                write!(f, "<{abbr}>")?;
            }
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_line() {
        macro_rules! test_case {
            ($input:literal -> [$($indent:ident),* $(,)?], $rest:literal) => {
                assert_eq!(
                    IndentToken::parse_line($input),
                    (IndentToken(vec![$(IndentChar::$indent),*]), $rest),
                )
            };
        }

        test_case!("" -> [], "");
        test_case!("abc" -> [], "abc");
        test_case!("\tabc" -> [Tab], "abc");
        test_case!("\t\tabc" -> [Tab, Tab], "abc");
        test_case!("  abc" -> [Space, Space], "abc");
        test_case!("    abc" -> [Space, Space, Space, Space], "abc");
        test_case!(" \t \tabc" -> [Space, Tab, Space, Tab], "abc");
        test_case!("  abc def ghi" -> [Space, Space], "abc def ghi");
        test_case!("\tabc def\tghi" -> [Tab], "abc def\tghi");
    }

    #[test]
    fn debug_fmt() {
        macro_rules! dbg_fmt {
            ($($name:ident),* $(,)?) => {{
                let indent_token = IndentToken(vec![$(IndentChar::$name),*]);
                format!("{indent_token:?}")
            }};
        }

        assert_eq!(dbg_fmt!(), "IndentToken []");
        assert_eq!(dbg_fmt!(Space), "IndentToken [<SPC>]");
        assert_eq!(dbg_fmt!(Tab), "IndentToken [<TAB>]");
        assert_eq!(
            dbg_fmt!(Space, Space, Space, Space),
            "IndentToken [<SPC✕4>]",
        );
        assert_eq!(
            dbg_fmt!(Space, Space, Tab, Tab, Tab, Space),
            "IndentToken [<SPC✕2><TAB✕3><SPC>]",
        );
    }
}
