use egg_text::{
    char_grid::{completed_char_grid, lazy_char_grid},
    CharAt, CharPos, CompletedCharGrid, EndOfLine, IterChar, IterLine, LazyCharGrid, LineAt, LnCol,
    LnNum, TryIterChar, TryIterLine,
};
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
    assert_eq!(char.value(), &'H');
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
    assert_eq!(char.value(), &'o');
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid
    assert_eq!(grid.loaded_char_count(), 7); // preloaded from partially_loaded_grid

    eprintln!("TEST 1:7 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(0, 6))
        .expect_err("char_at 1:7");
    assert_eq!(error, lazy_char_grid::CharAtLnColError::ColumnOutOfBound);

    eprintln!("TEST 2:1");
    let char = grid
        .char_at(LnCol::from_pred_counts(1, 0))
        .expect("char_at 2:1");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), "Hello,\n".len());
    assert_eq!(char.value(), &'I');
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
    assert_eq!(char.value(), &'❤');
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 20);

    eprintln!("TEST 2:13 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(1, 12))
        .expect_err("char_at 2:13");
    assert_eq!(error, lazy_char_grid::CharAtLnColError::ColumnOutOfBound);

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
    assert_eq!(char.value(), &'🥚');
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST 4:37 (expect error)");
    let error = grid
        .char_at(LnCol::from_pred_counts(3, 36))
        .expect_err("char_at 4:37");
    assert_eq!(error, lazy_char_grid::CharAtLnColError::ColumnOutOfBound);

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
    assert_eq!(char.value(), &'H');
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
    assert_eq!(char.value(), &'❤');
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());
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

#[test]
fn lazy_try_iter_char() {
    let mut acc = String::new();
    for (index, char) in partially_loaded_grid().try_iter_char().enumerate() {
        let char = char.unwrap_or_else(|error| panic!("attempt at index {index} failed: {error}"));
        let expected_char_pos = CharPos::from_pred_count(index);
        let received_char_pos = char.pos();
        eprintln!("{char:?}; expecting {expected_char_pos:?}, received {received_char_pos:?}");
        assert_eq!(received_char_pos, expected_char_pos);
        acc += char.to_string().as_str();
    }
    eprintln!("ACTUAL:\n{acc}\n");
    assert_eq!(acc, SRC_TEXT);
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

#[test]
fn completed_char_at_char_pos() {
    let grid = completed_grid();
    let char_count = dbg!(grid.char_count());

    eprintln!("TEST positive case");
    let mut acc = String::new();
    for (index, expected) in (0..char_count).zip(grid.iter_char()) {
        let char_pos = dbg!(CharPos::from_pred_count(index));
        let received = grid.char_at(char_pos).expect("get char");
        assert_eq!(received.to_string(), expected.to_string());
        assert_eq!(received.coord(), expected.coord());
        assert_eq!(received.pos(), expected.pos());
        assert_eq!(
            received.offset_from_ln_start(),
            expected.offset_from_ln_start(),
        );
        assert_eq!(
            received.offset_from_doc_start(),
            expected.offset_from_doc_start(),
        );
        acc += received.to_string().as_str();
    }

    eprintln!("TEST accumulation");
    assert_eq!(acc, SRC_TEXT);

    eprintln!("TEST out of bound");
    let char_pos = dbg!(CharPos::from_pred_count(char_count));
    let error = grid.char_at(char_pos).expect_err("should be out of bound");
    assert_eq!(error, completed_char_grid::CharAtCharPosError::OutOfBound);
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

#[test]
fn completed_iter_char() {
    let mut acc = String::new();
    for (index, char) in completed_grid().iter_char().enumerate() {
        let expected_char_pos = CharPos::from_pred_count(index);
        let received_char_pos = char.pos();
        eprintln!("{char:?}; expecting {expected_char_pos:?}, received {received_char_pos:?}");
        assert_eq!(received_char_pos, expected_char_pos);
        acc += char.to_string().as_str();
    }
    eprintln!("ACTUAL:\n{acc}\n");
    assert_eq!(acc, SRC_TEXT);
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
