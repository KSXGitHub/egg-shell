use egg_text::{
    char_grid::lazy_char_grid, CharAt, CharCoord, LazyCharGrid, LineAt, LineNumber, SliceFrom,
};
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
fn lazy_slice_from_char_at() {
    let grid = partially_loaded_grid();

    eprintln!("create the slice");
    let slice = grid
        .slice_from(CharCoord::from_pred_counts(1, 3))
        .expect("slice 2:4");
    let slice = slice
        .slice_from(CharCoord::from_pred_counts(1, 2))
        .expect("slice 2:3");

    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid

    eprintln!("TEST slice 2:4 -> slice 2:3 -> char_at 1:1");
    let char = slice
        .char_at(CharCoord::from_pred_counts(0, 0))
        .expect("char_at 1:1");

    assert_eq!(
        grid.data().loaded_text(),
        "Hello,\nI ❤️ Rust 🦀,\r\nSo I use it to create a programming language,\n",
    );

    assert_eq!(char.value(), &'u');
    assert_eq!(char.coord(), CharCoord::from_pred_counts(1 + 1, 2 + 3));

    eprintln!("TEST slice 2:4 -> slice 2:3 -> char_at 2:5");
    let char = slice
        .char_at(CharCoord::from_pred_counts(1, 4))
        .expect("char_at 2:5");

    assert_eq!(grid.data().loaded_text(), SRC_TEXT);

    assert_eq!(char.value(), &'a');
    assert_eq!(
        char.coord(),
        CharCoord::from_pred_counts(1 + 1 + 1, 2 + 3 + 4),
    );
}

#[test]
fn lazy_slice_from_line_at() {
    let grid = partially_loaded_grid();

    eprintln!("create the slice");
    let slice = grid
        .slice_from(CharCoord::from_pred_counts(1, 3))
        .expect("slice 2:4");
    let slice = slice
        .slice_from(CharCoord::from_pred_counts(1, 2))
        .expect("slice 2:3");

    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid

    eprintln!("TEST slice 2:4 -> slice 2:3 -> line_at 1");
    let line = slice
        .line_at(LineNumber::from_pred_count(0))
        .expect("line_at 1");

    assert_eq!(
        grid.data().loaded_text(),
        "Hello,\nI ❤️ Rust 🦀,\r\nSo I use it to create a programming language,\n",
    );
    assert_eq!(
        line.to_string(),
        "So I use it to create a programming language,\n",
    );
}