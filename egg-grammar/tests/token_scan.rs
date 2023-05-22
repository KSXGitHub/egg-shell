use egg_grammar::token::Scan;
use pretty_assertions::assert_eq;

macro_rules! test_snapshot {
    ($tokens:expr, $path:literal) => {{
        eprintln!("TEST: debug format snapshot");
        let received = format!("{:#?}", &$tokens);
        let expected = include_str!($path);
        assert_eq!(received.trim(), expected.trim());
    }};
}

macro_rules! test_ln_text {
    ($tokens:expr, $text:expr) => {{
        eprintln!("TEST: source of each TokenLine");
        let received: Vec<_> = $tokens.iter().map(|item| item.ln_text).collect();
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
