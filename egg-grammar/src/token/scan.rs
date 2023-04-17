/// Token scanner.
#[derive(Debug)]
pub struct Scan<'a> {
    text: &'a str,
    state: State,
}

/// State of the scanner.
#[derive(Debug, Default)]
struct State {
    scanned_lines: usize,
    context: Context,
}

/// Context of the scanner.
#[derive(Debug, Default)]
enum Context {
    #[default]
    Outermost,
}

impl<'a> Scan<'a> {
    /// Start scanning text for tokens.
    pub fn new(text: &'a str) -> Self {
        let state = State::default();
        Scan { text, state }
    }
}
