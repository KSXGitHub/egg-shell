/// Check if a character can be in a decimal, binary, or octal token.
///
/// **Why include 0-9 in binary and octal?**
/// > This function is to be used in the token level, which isn't designed to
/// > emit errors. Therefore, the token will include invalid digits and let the
/// > sematic layer handle them.
pub const fn is_number_body(char: &char) -> bool {
    matches!(char, '0'..='9' | '_')
}

/// Parse a number with prefix.
///
/// It is assumed that the resulting number body contains only ASCII characters.
pub fn parse_prefixed_number<'input, 'prefix, VerifyBody>(
    input: &'input str,
    prefix: &'prefix str,
    is_number_body: VerifyBody,
) -> Option<(&'input str, &'input str)>
where
    VerifyBody: Fn(&char) -> bool,
{
    let input = input.strip_prefix(prefix)?;
    if input.is_empty() {
        return None;
    }
    let body_size = input.chars().take_while(is_number_body).count(); // digit always has len_utf8 = 1
    let body = &input[..body_size];
    let rest = &input[body_size..];
    Some((body, rest))
}

/// Combine two boolean functions with an AND operator.
const fn combine_condition<X, F, G>(f: F, g: G) -> impl Fn(&X) -> bool
where
    F: Fn(&X) -> bool,
    G: Fn(&X) -> bool,
{
    move |x| f(x) && g(x)
}

/// Declare a function that can be used as a verifier in [`split_hbt_ascii`].
#[macro_export]
macro_rules! char_matcher {
    ($(#[$attr:meta])* $visibility:vis $name:ident => $pattern:pat) => {
        $(#[$attr])*
        $visibility const fn $name(char: &char) -> bool {
            matches!(char, $pattern)
        }
    };
}

/// Extract an ASCII sequence of string whose first char, last char, and middle chars
/// have 3 different requirements.
///
/// **Note:** The 3 character verifiers will only run on ASCII characters.
///
/// **Return:**
/// * The first item of the tuple is the resulting sequence of string.
/// * The second item of the tuple is the remaining part of the input string.
pub fn split_hbt_ascii<VerifyHead, VerifyBody, VerifyTail>(
    input: &str,
    is_head: VerifyHead,
    is_body: VerifyBody,
    is_tail: VerifyTail,
) -> (&'_ str, &'_ str)
where
    VerifyHead: Fn(&char) -> bool,
    VerifyBody: Fn(&char) -> bool,
    VerifyTail: Fn(&char) -> bool,
{
    // Ensure that the verifiers only execute on ASCII characters.
    // If is_ascii is redundant, the compiler will optimize it away.
    let is_head = combine_condition(char::is_ascii, is_head);
    let is_body = combine_condition(char::is_ascii, is_body);
    let is_tail = combine_condition(char::is_ascii, is_tail);

    let mut iter = input.chars();

    let Some(first_char) = iter.next() else {
        return ("", input);
    };
    if !is_head(&first_char) {
        return ("", input);
    }

    let first_char_len = 1; // because it is an ascii character.
    debug_assert_eq!(first_char_len, first_char.len_utf8());
    let tail_size = iter.take_while(is_body).count(); // ascii char has len_utf8 = 1
    let end_offset = first_char_len + tail_size;

    let word = &input[..end_offset];
    let last_char = word.chars().next_back().expect("word is not empty");

    if is_tail(&last_char) {
        let rest = &input[end_offset..];
        (word, rest)
    } else {
        let end_offset = end_offset - 1; // it's ascii, no needs to worry about panic
        let word = &input[..end_offset];
        let rest = &input[end_offset..];
        (word, rest)
    }
}
