use egg_grammar::token::{Scan, TokenLine};
use pretty_assertions::assert_eq;

macro_rules! title {
    ($title:literal) => {{
        eprintln!();
        eprintln!("TEST: {}", $title);
    }};
}

macro_rules! test_snapshot {
    ($tokens:expr, $path:literal) => {{
        title!("debug format snapshot");
        let received = format!("{:#?}", &$tokens);
        let expected = include_str!($path);
        assert_eq!(received.trim(), expected.trim());
    }};
}

macro_rules! test_ln_text {
    ($tokens:expr, $text:expr) => {{
        title!("source of each TokenLine");
        let received: Vec<_> = $tokens
            .iter()
            .flat_map(TokenLine::all_src_text)
            .copied()
            .collect();
        let expected: Vec<_> = $text.lines().collect();
        assert_eq!(&received, &expected);
    }};
}

#[test]
fn hello_world() {
    let text = include_str!("fixtures/hello-world.egg");
    let tokens: Vec<_> = dbg!(Scan::new(text).collect());
    test_snapshot!(tokens, "snapshots/token-scan/hello-world.txt");
    test_ln_text!(tokens, text);
}

#[test]
fn multi_line() {
    let text = include_str!("fixtures/multi-line.egg");
    let tokens: Vec<_> = dbg!(Scan::new(text).collect());
    test_snapshot!(tokens, "snapshots/token-scan/multi-line.txt");
    test_ln_text!(tokens, text);
}
