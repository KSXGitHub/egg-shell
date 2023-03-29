use super::IndentChar;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator};
use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};

/// Token of indentation.
#[derive(Clone, PartialEq, Eq, AsMut, AsRef, Deref, DerefMut, From, Into, IntoIterator)]
pub struct IndentToken(Vec<IndentChar>);

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
