use super::IndentChar;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator};
use itertools::Itertools;
use split_first_char::split_first_char;
use std::fmt::{self, Debug, Display, Formatter};

/// Token of indentation.
#[derive(Clone, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator)]
pub struct IndentToken(Vec<IndentChar>);

impl IndentToken {
    /// Parse a line of text into a pair of indentation and remaining string.
    ///
    /// **Notes:**
    /// * `line` is assumed to not contain any EOL characters.
    pub fn parse(mut line: &str) -> (Self, &'_ str) {
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

    /// Check if the indent is the start of another indent.
    pub fn is_start_of(&self, other: &[IndentChar]) -> bool {
        other.starts_with(self)
    }

    /// Check if the indent is the shorter start of another indent.
    pub fn is_shorter_start_of(&self, other: &[IndentChar]) -> bool {
        self.len() > other.len() && self.is_start_of(other)
    }
}

impl Display for IndentToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for char in self.iter() {
            write!(f, "{}", char)?
        }
        Ok(())
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
            ($input:literal -> [$($indent:ident),* $(,)?], $rest:literal) => {{
                eprintln!("TEST: {:?}", $input);
                assert_eq!(
                    IndentToken::parse($input),
                    (IndentToken(vec![$(IndentChar::$indent),*]), $rest),
                );
            }};
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
    fn display_fmt() {
        macro_rules! str_fmt {
            ($($name:ident),* $(,)?) => {
                IndentToken(vec![$(IndentChar::$name),*]).to_string()
            };
        }

        assert_eq!(str_fmt!(), "");
        assert_eq!(str_fmt!(Space), " ");
        assert_eq!(str_fmt!(Tab), "\t");
        assert_eq!(str_fmt!(Space, Space, Space, Space), " ".repeat(4),);
        assert_eq!(
            str_fmt!(Space, Space, Tab, Tab, Tab, Space),
            format!("{spc}{spc}{tab}{tab}{tab}{spc}", spc = " ", tab = "\t"),
        );
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
