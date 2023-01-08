use crate::{
    char_grid::completed_char_grid, CharAt, CharPos, CompletedCharGrid, LazyCharGrid, LnCol,
};
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
fn completed_char_at_ln_col() {
    let grid = completed_grid();

    eprintln!("TEST 1:1");
    let char = grid
        .char_at(LnCol::from_pred_counts(0, 0))
        .expect("char_at 1:1");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 0);
    assert_eq!(char.value(), &'H');

    eprintln!("TEST 1:5");
    let char = grid
        .char_at(LnCol::from_pred_counts(0, 4))
        .expect("char_at 1:5");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 4));
    assert_eq!(char.pos(), CharPos::from_pred_count(4));
    assert_eq!(char.offset_from_ln_start(), 4);
    assert_eq!(char.offset_from_doc_start(), "Hell".len());
    assert_eq!(char.value(), &'o');

    eprintln!("TEST 1:7 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(0, 6))
        .expect_err("char_at 1:7");
    assert_eq!(
        error,
        completed_char_grid::CharAtLnColError::ColumnOutOfBound
    );

    eprintln!("TEST 2:1");
    let char = grid
        .char_at(LnCol::from_pred_counts(1, 0))
        .expect("char_at 2:1");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), "Hello,\n".len());
    assert_eq!(char.value(), &'I');

    eprintln!("TEST 2:3");
    let char = grid
        .char_at(LnCol::from_pred_counts(1, 2))
        .expect("char_at 2:3");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 2));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1 + 2));
    assert_eq!(char.offset_from_ln_start(), 2);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ".len());
    assert_eq!(char.value(), &'❤');

    eprintln!("TEST 2:13 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(1, 12))
        .expect_err("char_at 2:13");
    assert_eq!(
        error,
        completed_char_grid::CharAtLnColError::ColumnOutOfBound
    );

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
    assert_eq!(char.value(), &'T');

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
    assert_eq!(char.value(), &'🥚');

    eprintln!("TEST 4:37 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(3, 36))
        .expect_err("char_at 4:37");
    assert_eq!(
        error,
        completed_char_grid::CharAtLnColError::ColumnOutOfBound
    );

    eprintln!("TEST 5:1 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(4, 0))
        .expect_err("char_at 5:1");
    assert_eq!(error, completed_char_grid::CharAtLnColError::LineOutOfBound);
}
