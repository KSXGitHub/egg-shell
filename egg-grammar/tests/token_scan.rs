use egg_grammar::token::{Scan, TokenLine};
use exec_diff::assert_eq_uni_diff;
use itertools::Itertools;
use pretty_assertions::assert_eq;

fn title(title: &str) {
    eprintln!();
    eprintln!("TEST: {}", title);
}

macro_rules! test_snapshot {
    ($tokens:expr, $path:literal) => {{
        title("Snapshot of the Debug format the tokens");
        let received = format!("{:#?}", &$tokens);
        let expected = include_str!($path);
        assert_eq_uni_diff(received.trim(), expected.trim());
    }};
}

fn test_ln_text(tokens: &[TokenLine<&str>], text: &str) {
    title("Equality between all_ln_text and the original source");
    let received: Vec<_> = tokens
        .iter()
        .flat_map(TokenLine::all_ln_text)
        .copied()
        .collect();
    let expected: Vec<_> = text.lines().collect();
    assert_eq!(&received, &expected);
}

fn test_src_text(tokens: &[TokenLine<&str>]) {
    title("Equality between ln_text and sum of src_text");
    for item in tokens {
        let ln_text = dbg!(item.ln_text);
        let indent = dbg!(item.indent.src_text);
        let middle = dbg!(item.middle.iter().map(|item| item.src_text).join(""));
        let ending = dbg!(item.ending.as_ref().map_or("", |item| item.src_text.0));
        let sum = dbg!(format!("{indent}{middle}{ending}"));
        assert_eq!(&sum, ln_text);
    }
}

#[test]
fn hello_world() {
    let text = include_str!("fixtures/hello-world.egg");
    let tokens: Vec<_> = dbg!(Scan::new(text).collect());
    test_snapshot!(tokens, "snapshots/token-scan/hello-world.txt");
    test_ln_text(&tokens, text);
    test_src_text(&tokens);
}

#[test]
fn multi_line() {
    let text = include_str!("fixtures/multi-line.egg");
    let tokens: Vec<_> = dbg!(Scan::new(text).collect());
    test_snapshot!(tokens, "snapshots/token-scan/multi-line.txt");
    test_ln_text(&tokens, text);
    test_src_text(&tokens);
}

#[test]
fn invalid_char_heart() {
    let text = "print 'hello world' with ❤️";
    let tokens: Vec<_> = dbg!(Scan::new(text).collect());
    test_snapshot!(tokens, "snapshots/token-scan/invalid-char-heart.txt");
    test_ln_text(&tokens, text);
    test_src_text(&tokens);
}

#[test]
fn invalid_char_nul() {
    let text = "print 'hello world'\0";
    let tokens: Vec<_> = dbg!(Scan::new(text).collect());
    test_snapshot!(tokens, "snapshots/token-scan/invalid-char-nul.txt");
    test_ln_text(&tokens, text);
    test_src_text(&tokens);
}
