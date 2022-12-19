use egg_grammar::CharTable;
use pretty_assertions::assert_eq;

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use to create a programming language,\n",
    "The language is called 'egg-shell' 🥚\n",
    "It's inspired by Rust 🦀",
    "It's going to be awesome!",
};

fn table() -> CharTable<impl Iterator<Item = char>> {
    CharTable::from_static_str(SRC_TEXT)
        .into_completed()
        .expect("load table")
}

#[test]
fn line_correctness() {
    let table = table();
    let received: Vec<_> = table
        .loaded_line_list()
        .iter()
        .map(|(segment, eol)| (segment.to_string(), eol))
        .collect();
    dbg!(&received);
    let expected = [];
    assert_eq!(received, expected);
}
