use crate::{char_grid::lazy_char_grid, LazyCharGrid, TryIterLine};
use pretty_assertions::assert_eq;

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use it to create a programming language,\n",
    "The language is called 'egg-shell' 🥚",
};

fn partially_loaded_grid() -> lazy_char_grid::LazyCharGridFromStr<'static> {
    let grid = LazyCharGrid::from_str(SRC_TEXT);
    let first_line = grid
        .load_line()
        .expect("load a line")
        .expect("there should be a line");
    assert_eq!(&first_line.text_without_eol(), "Hello,");
    let next_3_chars = (0..3).map(|index| match grid.load_char() {
        Err(error) => panic!("load_char fails at {index}: {error}"),
        Ok(lazy_char_grid::LoadCharReport::Char(char)) => char,
        Ok(report) => {
            panic!("load_char at {index} does not return a char, but instead: {report:?}")
        }
    });
    assert_eq!(next_3_chars.collect::<Vec<_>>(), ['I', ' ', '❤']);
    grid
}

#[test]
fn lazy_try_iter_line() {
    let grid = partially_loaded_grid();
    let mut acc = String::new();
    for (index, line) in grid.try_iter_line().enumerate() {
        let line = line.unwrap_or_else(|error| panic!("attempt at line {index} failed: {error}"));
        dbg!(line);
        let text_without_eol = dbg!(line.text_without_eol());
        let eol = dbg!(line.eol());
        text_without_eol.run(|text| acc += text);
        acc += eol.as_ref();
    }
    eprintln!("ACTUAL:\n{acc}\n");
    assert_eq!(acc, SRC_TEXT);
}
