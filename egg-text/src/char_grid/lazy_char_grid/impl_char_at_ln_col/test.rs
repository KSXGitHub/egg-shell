use crate::{char_grid::lazy_char_grid, CharAt, CharPos, LazyCharGrid, LnCol};
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
fn lazy_char_at_ln_col() {
    let grid = partially_loaded_grid();

    eprintln!("TEST 1:1");
    let char = grid
        .char_at(LnCol::from_pred_counts(0, 0))
        .expect("char_at 1:1");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 0);
    assert_eq!(char.value().to_string(), "H");
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid
    assert_eq!(grid.loaded_char_count(), 7); // preloaded from partially_loaded_grid

    eprintln!("TEST 1:5");
    let char = grid
        .char_at(LnCol::from_pred_counts(0, 4))
        .expect("char_at 1:5");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 4));
    assert_eq!(char.pos(), CharPos::from_pred_count(4));
    assert_eq!(char.offset_from_ln_start(), 4);
    assert_eq!(char.offset_from_doc_start(), "Hell".len());
    assert_eq!(char.value().to_string(), "o");
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid
    assert_eq!(grid.loaded_char_count(), 7); // preloaded from partially_loaded_grid

    // TODO: eprintln!("TEST 1:7");
    // TODO: eprintln!("TEST 1:8 (expect error)");

    eprintln!("TEST 2:1");
    let char = grid
        .char_at(LnCol::from_pred_counts(1, 0))
        .expect("char_at 2:1");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), "Hello,\n".len());
    assert_eq!(char.value().to_string(), "I");
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 20);

    eprintln!("TEST 2:3");
    let char = grid
        .char_at(LnCol::from_pred_counts(1, 2))
        .expect("char_at 2:3");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 2));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1 + 2));
    assert_eq!(char.offset_from_ln_start(), 2);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ".len());
    assert_eq!(char.value().to_string(), "❤");
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 20);

    // TODO: eprintln!("TEST 2:13");
    // TODO: eprintln!("TEST 2:14 (expect error)");

    eprintln!("TEST 4:1");
    let char = grid
        .char_at(LnCol::from_pred_counts(3, 0))
        .expect("char_at 4:1");
    assert_eq!(char.coord(), LnCol::from_pred_counts(3, 0));
    assert_eq!(
        char.pos(),
        CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1),
    );
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 74);
    assert_eq!(char.value().to_string(), "T");
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST 4:36");
    let char = grid
        .char_at(LnCol::from_pred_counts(3, 35))
        .expect("char_at 4:36");
    assert_eq!(char.coord(), LnCol::from_pred_counts(3, 35));
    assert_eq!(
        char.pos(),
        CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1 + 35),
    );
    assert_eq!(char.offset_from_ln_start(), 35);
    assert_eq!(char.offset_from_doc_start(), 109);
    assert_eq!(char.value().to_string(), "🥚");
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    // TODO: eprintln!("TEST 4:37");
    // TODO: eprintln!("TEST 4:38 (expect error)");

    eprintln!("TEST 5:1 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(4, 0))
        .expect_err("char_at 5:1");
    assert_eq!(error, lazy_char_grid::CharAtLnColError::LineOutOfBound);

    eprintln!("TEST 1:1 (again)");
    let char = grid
        .char_at(LnCol::from_pred_counts(0, 0))
        .expect("char_at 1:1");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 0);
    assert_eq!(char.value().to_string(), "H");
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST 2:3 (again)");
    let char = grid
        .char_at(LnCol::from_pred_counts(1, 2))
        .expect("char_at 2:3");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 2));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1 + 2));
    assert_eq!(char.offset_from_ln_start(), 2);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ".len());
    assert_eq!(char.value().to_string(), "❤");
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());
}
