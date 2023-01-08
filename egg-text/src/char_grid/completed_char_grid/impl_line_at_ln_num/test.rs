use crate::{
    char_grid::completed_char_grid, CharPos, CompletedCharGrid, EndOfLine, LazyCharGrid, LineAt,
    LnCol, LnNum,
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
fn completed_line_at() {
    let grid = completed_grid();

    eprintln!("TEST 1");
    let line = grid.line_at(LnNum::from_pred_count(0)).expect("line_at 1");
    assert_eq!(line.slice().first_char_pos(), CharPos::from_pred_count(0));
    assert_eq!(
        line.slice().first_char_coord(),
        LnCol::from_pred_counts(0, 0),
    );
    assert_eq!(line.eol(), EndOfLine::LF);
    assert_eq!(line.text_without_eol(), "Hello,");

    eprintln!("TEST 2");
    let line = grid.line_at(LnNum::from_pred_count(1)).expect("line_at 2");
    let ln_count = 1;
    assert_eq!(
        line.slice().first_char_pos(),
        SRC_TEXT
            .lines()
            .take(ln_count)
            .map(str::chars)
            .map(Iterator::count)
            .sum::<usize>()
            .pipe(CharPos::from_pred_count)
            .advance_by(ln_count),
    );
    assert_eq!(
        line.slice().first_char_coord(),
        LnCol::from_pred_counts(1, 0),
    );
    assert_eq!(line.eol(), EndOfLine::CRLF);
    assert_eq!(line.text_without_eol(), "I ❤️ Rust 🦀,");

    eprintln!("TEST 4");
    let line = grid.line_at(LnNum::from_pred_count(3)).expect("line_at 4");
    let ln_count = 3;
    assert_eq!(
        line.slice().first_char_pos(),
        SRC_TEXT
            .lines()
            .take(ln_count)
            .map(str::chars)
            .map(Iterator::count)
            .sum::<usize>()
            .pipe(CharPos::from_pred_count)
            .advance_by(ln_count),
    );
    assert_eq!(
        line.slice().first_char_coord(),
        LnCol::from_pred_counts(3, 0),
    );
    assert_eq!(line.eol(), EndOfLine::EOF);
    assert_eq!(
        line.text_without_eol(),
        "The language is called 'egg-shell' 🥚",
    );

    eprintln!("TEST 5 (expect error)");
    let error = grid
        .line_at(LnNum::from_pred_count(4))
        .expect_err("line_at 5");
    assert_eq!(error, completed_char_grid::LineAtError::OutOfBound);
}
