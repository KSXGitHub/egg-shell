use egg_text::{
    char_grid::lazy_char_grid, CharCoord, EndOfLine, LazyCharGrid, LoadCharAt, LoadLineAt, Ordinal,
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use std::{convert::Infallible, str::Chars};

const SRC_TEXT: &str = concat! {
    "Hello,\n",
    "I ❤️ Rust 🦀,\r\n",
    "So I use it to create a programming language,\n",
    "The language is called 'egg-shell' 🥚",
};

fn partially_loaded_grid() -> LazyCharGrid<impl Iterator<Item = Result<char, Infallible>>> {
    let mut grid = LazyCharGrid::from_str(SRC_TEXT);
    let first_line = grid
        .load_line()
        .expect("load a line")
        .expect("there should be a line")
        .text_without_eol(&grid);
    assert_eq!(first_line, "Hello,");
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
fn lazy_load_char_at() {
    let mut grid = partially_loaded_grid();

    eprintln!("TEST 1:1");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(0, 0))
        .expect("char_at 1:1");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(0, 0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 0);
    assert_eq!(char.value(), &'H');
    assert_eq!(grid.loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid
    assert_eq!(grid.loaded_char_count(), 10); // preloaded from partially_loaded_grid

    eprintln!("TEST 1:5");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(0, 4))
        .expect("char_at 1:5");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(0, 4));
    assert_eq!(char.offset_from_ln_start(), 4);
    assert_eq!(char.offset_from_doc_start(), "Hell".len());
    assert_eq!(char.value(), &'o');
    assert_eq!(grid.loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid
    assert_eq!(grid.loaded_char_count(), 10); // preloaded from partially_loaded_grid

    eprintln!("TEST 1:7 (expect error)");
    let error = grid
        .load_char_at(CharCoord::from_pred_counts(0, 6))
        .expect_err("char_at 1:7");
    assert_eq!(error, lazy_char_grid::CharAtError::ColumnOutOfBound);

    eprintln!("TEST 2:1");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(1, 0))
        .expect("char_at 2:1");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(1, 0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), "Hello,\n".len());
    assert_eq!(char.value(), &'I');
    assert_eq!(grid.loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 21);

    eprintln!("TEST 2:3");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(1, 2))
        .expect("char_at 2:3");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(1, 2));
    assert_eq!(char.offset_from_ln_start(), 2);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ".len());
    assert_eq!(char.value(), &'❤');
    assert_eq!(grid.loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 21);

    eprintln!("TEST 2:13 (expect error)");
    let error = grid
        .load_char_at(CharCoord::from_pred_counts(1, 12))
        .expect_err("char_at 2:13");
    assert_eq!(error, lazy_char_grid::CharAtError::ColumnOutOfBound);

    eprintln!("TEST 4:1");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(3, 0))
        .expect("char_at 4:1");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(3, 0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 74);
    assert_eq!(char.value(), &'T');
    assert_eq!(grid.loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST 4:36");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(3, 35))
        .expect("char_at 4:36");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(3, 35));
    assert_eq!(char.offset_from_ln_start(), 35);
    assert_eq!(char.offset_from_doc_start(), 109);
    assert_eq!(char.value(), &'🥚');
    assert_eq!(grid.loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST 4:37 (expect error)");
    let error = grid
        .load_char_at(CharCoord::from_pred_counts(3, 36))
        .expect_err("char_at 4:37");
    assert_eq!(error, lazy_char_grid::CharAtError::ColumnOutOfBound);

    eprintln!("TEST 5:1 (expect error)");
    let error = grid
        .load_char_at(CharCoord::from_pred_counts(4, 0))
        .expect_err("char_at 5:1");
    assert_eq!(error, lazy_char_grid::CharAtError::LineOutOfBound);

    eprintln!("TEST 1:1 (again)");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(0, 0))
        .expect("char_at 1:1");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(0, 0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 0);
    assert_eq!(char.value(), &'H');
    assert_eq!(grid.loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST 2:3 (again)");
    let char = grid
        .load_char_at(CharCoord::from_pred_counts(1, 2))
        .expect("char_at 2:3");
    assert_eq!(char.coord(), CharCoord::from_pred_counts(1, 2));
    assert_eq!(char.offset_from_ln_start(), 2);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ".len());
    assert_eq!(char.value(), &'❤');
    assert_eq!(grid.loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());
}

#[test]
fn lazy_load_line_at() {
    let mut grid = partially_loaded_grid();

    eprintln!("TEST 1");
    let line = grid
        .load_line_at(Ordinal::from_pred_count(0))
        .expect("line_at 1");
    assert_eq!(line.slice().first_char_pos(), Ordinal::from_pred_count(0));
    assert_eq!(
        line.slice().first_char_coord(),
        CharCoord::from_pred_counts(0, 0),
    );
    assert_eq!(line.eol(), EndOfLine::LF);
    assert_eq!(line.text_without_eol(&grid), "Hello,");
    assert_eq!(grid.loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid

    eprintln!("TEST 2");
    let line = grid
        .load_line_at(Ordinal::from_pred_count(1))
        .expect("line_at 2");
    assert_eq!(
        line.slice().first_char_pos(),
        "Hello,".chars().count().pipe(Ordinal::from_pred_count),
    );
    assert_eq!(
        line.slice().first_char_coord(),
        CharCoord::from_pred_counts(1, 0),
    );
    assert_eq!(line.eol(), EndOfLine::CRLF);
    assert_eq!(line.text_without_eol(&grid), "I ❤️ Rust 🦀,");
    assert_eq!(grid.loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");

    eprintln!("TEST 4");
    let line = grid
        .load_line_at(Ordinal::from_pred_count(3))
        .expect("line_at 4");
    assert_eq!(
        line.slice().first_char_pos(),
        SRC_TEXT
            .lines()
            .take(3)
            .map(str::chars)
            .map(Chars::count)
            .sum::<usize>()
            .pipe(Ordinal::from_pred_count),
    );
    assert_eq!(
        line.slice().first_char_coord(),
        CharCoord::from_pred_counts(3, 0),
    );
    assert_eq!(line.eol(), EndOfLine::EOF);
    assert_eq!(
        line.text_without_eol(&grid),
        "The language is called 'egg-shell' 🥚",
    );
    assert_eq!(grid.loaded_text(), SRC_TEXT);

    eprintln!("TEST 5 (expect error)");
    let error = grid
        .load_line_at(Ordinal::from_pred_count(4))
        .expect_err("line_at 5");
    assert_eq!(error, lazy_char_grid::LineAtError::OutOfBound);

    eprintln!("TEST 1 (again)");
    let line = grid
        .load_line_at(Ordinal::from_pred_count(0))
        .expect("line_at 1");
    assert_eq!(line.slice().first_char_pos(), Ordinal::from_pred_count(0));
    assert_eq!(
        line.slice().first_char_coord(),
        CharCoord::from_pred_counts(0, 0),
    );
    assert_eq!(line.eol(), EndOfLine::LF);
    assert_eq!(line.text_without_eol(&grid), "Hello,");
    assert_eq!(grid.loaded_text(), SRC_TEXT);
}
