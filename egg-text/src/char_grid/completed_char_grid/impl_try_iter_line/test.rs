use crate::{CompletedCharGrid, IterLine, LazyCharGrid};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use it to create a programming language,\n",
    "The language is called 'egg-shell' 🥚",
};

fn completed_grid() -> CompletedCharGrid {
    SRC_TEXT
        .pipe(LazyCharGrid::from_str)
        .into_completed()
        .expect("load grid")
}

#[test]
fn completed_iter_line() {
    let grid = completed_grid();
    let mut acc = String::new();
    for line in grid.iter_line() {
        dbg!(line);
        let text_without_eol = dbg!(line.text_without_eol());
        let eol = dbg!(line.eol());
        acc += text_without_eol;
        acc += eol.as_ref();
    }
    eprintln!("ACTUAL:\n{acc}\n");
    assert_eq!(acc, SRC_TEXT);
}
