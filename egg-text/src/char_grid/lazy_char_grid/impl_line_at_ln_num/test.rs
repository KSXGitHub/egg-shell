use crate::{char_grid::lazy_char_grid, CharPos, EndOfLine, LazyCharGrid, LineAt, LnCol, LnNum};
use pipe_trait::Pipe;
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
fn lazy_line_at() {
    let grid = partially_loaded_grid();

    eprintln!("TEST 1");
    let line = grid.line_at(LnNum::from_pred_count(0)).expect("line_at 1");
    assert_eq!(line.slice().first_char_pos(), CharPos::from_pred_count(0));
    assert_eq!(
        line.slice().first_char_coord(),
        LnCol::from_pred_counts(0, 0),
    );
    assert_eq!(line.eol(), EndOfLine::LF);
    assert_eq!(&line.text_without_eol(), "Hello,");
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid

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
    assert_eq!(&line.text_without_eol(), "I ❤️ Rust 🦀,");
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");

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
        &line.text_without_eol(),
        "The language is called 'egg-shell' 🥚",
    );
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);

    eprintln!("TEST 5 (expect error)");
    let error = grid
        .line_at(LnNum::from_pred_count(4))
        .expect_err("line_at 5");
    assert_eq!(error, lazy_char_grid::LineAtError::OutOfBound);

    eprintln!("TEST 1 (again)");
    let line = grid.line_at(LnNum::from_pred_count(0)).expect("line_at 1");
    assert_eq!(line.slice().first_char_pos(), CharPos::from_pred_count(0));
    assert_eq!(
        line.slice().first_char_coord(),
        LnCol::from_pred_counts(0, 0),
    );
    assert_eq!(line.eol(), EndOfLine::LF);
    assert_eq!(&line.text_without_eol(), "Hello,");
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
}
