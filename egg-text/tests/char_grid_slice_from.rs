#![allow(clippy::identity_op)] // allow expressing 0 + n

use egg_text::{char_grid::lazy_char_grid, CharAt, LazyCharGrid, LineAt, LnCol, LnNum, SliceFrom};
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
fn lazy_slice_from_ln_col_char_at() {
    macro_rules! test_char {
        ($char:expr, $expected_coord:expr) => {{
            assert_eq!(
                *$char.value(),
                SRC_TEXT
                    .lines()
                    .nth($expected_coord.line.pred_count())
                    .unwrap()
                    .chars()
                    .nth($expected_coord.column.pred_count())
                    .unwrap(),
            );
            assert_eq!($char.coord(), $expected_coord);
        }};
    }

    let grid = partially_loaded_grid();

    eprintln!("create the slice");
    let start1 = LnCol::from_pred_counts(1, 3);
    let start2 = LnCol::from_pred_counts(1, 2);
    let slice = grid
        .slice_from(start1)
        .expect("slice 2:4")
        .slice_from(start2)
        .expect("slice 2:3");

    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid

    eprintln!("TEST slice 2:4 -> slice 2:3 -> char_at 1:1");
    let coord = LnCol::from_pred_counts(0, 0);
    let char = slice.char_at(coord).expect("char_at 1:1");

    assert_eq!(
        grid.data().loaded_text(),
        "Hello,\nI ❤️ Rust 🦀,\r\nSo I use it to create a programming language,\n",
    );

    let expected_coord = LnCol::from_pred_counts(
        start1.line.pred_count() + start2.line.pred_count() + coord.line.pred_count(),
        0 + start2.column.pred_count() + coord.column.pred_count(),
    );
    test_char!(char, expected_coord);

    eprintln!("TEST slice 2:4 -> slice 2:3 -> char_at 1:7");
    let coord = LnCol::from_pred_counts(0, 6);
    let char = slice.char_at(coord).expect("char_at 1:7");

    assert_eq!(
        grid.data().loaded_text(),
        "Hello,\nI ❤️ Rust 🦀,\r\nSo I use it to create a programming language,\n",
    );

    let expected_coord = LnCol::from_pred_counts(
        start1.line.pred_count() + start2.line.pred_count() + coord.line.pred_count(),
        0 + start2.column.pred_count() + coord.column.pred_count(),
    );
    test_char!(char, expected_coord);

    eprintln!("TEST slice 2:4 -> slice 2:3 -> char_at 2:5");
    let coord = LnCol::from_pred_counts(1, 4);
    let char = slice.char_at(coord).expect("char_at 2:5");

    assert_eq!(grid.data().loaded_text(), SRC_TEXT);

    let expected_coord = LnCol::from_pred_counts(
        start1.line.pred_count() + start2.line.pred_count() + coord.line.pred_count(),
        0 + 0 + coord.column.pred_count(),
    );
    test_char!(char, expected_coord);
}

#[test]
fn lazy_slice_from_ln_col_line_at() {
    let grid = partially_loaded_grid();

    eprintln!("create the slice");
    let slice = grid
        .slice_from(LnCol::from_pred_counts(1, 3))
        .expect("slice 2:4")
        .slice_from(LnCol::from_pred_counts(1, 2))
        .expect("slice 2:3");

    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid

    eprintln!("TEST slice 2:4 -> slice 2:3 -> line_at 1");
    let line = slice.line_at(LnNum::from_pred_count(0)).expect("line_at 1");

    assert_eq!(
        grid.data().loaded_text(),
        "Hello,\nI ❤️ Rust 🦀,\r\nSo I use it to create a programming language,\n",
    );
    assert_eq!(
        line.to_string(),
        "So I use it to create a programming language,\n",
    );
}
