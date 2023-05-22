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

#[test]
fn hello_world() {
    let text = include_str!("fixtures/hello-world.egg");
    let tokens: Vec<_> = dbg!(Scan::new(text).collect());
    test_snapshot!(tokens, "snapshots/token-scan/hello-world.txt");
}
