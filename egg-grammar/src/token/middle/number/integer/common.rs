/// Parse a number with prefix.
///
/// It is assumed that the resulting number body contains only ASCII characters.
pub fn parse_prefixed_number<'input, 'prefix>(
    input: &'input str,
    prefix: &'prefix str,
    is_number_body: impl Fn(&char) -> bool,
) -> Option<(&'input str, &'input str)> {
    let input = input.strip_prefix(prefix)?;
    if input.is_empty() {
        return None;
    }
    let body_size = input.chars().take_while(is_number_body).count(); // digit always has len_utf8 = 1
    let body = &input[..body_size];
    let remaining = &input[body_size..];
    Some((body, remaining))
}
