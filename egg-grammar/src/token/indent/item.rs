use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::fmt::{self, Display, Formatter};

/// Character for indentation. Either a space or a tab.
///
/// Use [`TryFrom<char>`] to create an `IndentChar`.
///
/// Use [`Into<char>`] to extract the original character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentChar {
    Space,
    Tab,
}

impl IndentChar {
    /// Get 3-letter abbreviation.
    pub(crate) const fn abbr(self) -> &'static str {
        match self {
            IndentChar::Space => "SPC",
            IndentChar::Tab => "TAB",
        }
    }
}

impl From<IndentChar> for char {
    fn from(input: IndentChar) -> Self {
        match input {
            IndentChar::Space => ' ',
            IndentChar::Tab => '\t',
        }
    }
}

/// Error when failing to [convert](TryFrom) a `char` to an [`IndentChar`].
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
#[display(fmt = "Cannot convert {_0:?} to an indentation")]
pub struct IndentCharParseError(#[error(not(source))] char);

impl IndentCharParseError {
    /// Get the original input.
    pub const fn input(self) -> char {
        self.0
    }
}

impl TryFrom<char> for IndentChar {
    type Error = IndentCharParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(IndentChar::Space),
            '\t' => Ok(IndentChar::Tab),
            _ => value.pipe(IndentCharParseError).pipe(Err),
        }
    }
}

impl Display for IndentChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        char::from(*self).fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn convert_indent_to_char() {
        assert_eq!(char::from(IndentChar::Space), ' ');
        assert_eq!(char::from(IndentChar::Tab), '\t');
    }

    #[test]
    fn convert_char_to_indent() {
        assert_eq!(' '.pipe(IndentChar::try_from).unwrap(), IndentChar::Space);
        assert_eq!('\t'.pipe(IndentChar::try_from).unwrap(), IndentChar::Tab);
        assert_eq!(
            'a'.pipe(IndentChar::try_from).unwrap_err(),
            IndentCharParseError('a'),
        );
    }

    #[test]
    fn convert_back_forth() {
        macro_rules! char_to_char {
            ($char:literal) => {
                assert_eq!(
                    $char.pipe(IndentChar::try_from).unwrap().pipe(char::from),
                    $char,
                );
            };
        }

        macro_rules! indent_to_indent {
            ($name:ident) => {
                assert_eq!(
                    IndentChar::$name
                        .pipe(char::from)
                        .pipe(IndentChar::try_from)
                        .unwrap(),
                    IndentChar::$name,
                );
            };
        }

        char_to_char!(' ');
        char_to_char!('\t');
        indent_to_indent!(Space);
        indent_to_indent!(Tab);
    }

    #[test]
    fn display_fmt() {
        assert_eq!(IndentChar::Space.pipe(char::from).to_string(), " ");
        assert_eq!(IndentChar::Tab.pipe(char::from).to_string(), "\t");
    }
}
