/// Split a string into a pair of first character and the rest.
pub fn split_first_char(text: &str) -> Option<(char, &'_ str)> {
    let mut iter = text.chars();
    let first = iter.next()?;
    let rest = iter.as_str();
    Some((first, rest))
}

#[cfg(test)]
mod test {
    use super::split_first_char;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_split_first_char() {
        assert_eq!(split_first_char("abc"), Some(('a', "bc")));
        assert_eq!(split_first_char("x"), Some(('x', "")));
        assert_eq!(split_first_char(""), None);
    }
}
