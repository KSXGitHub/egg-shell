use crate::{
    char_grid::lazy_char_grid, CharAt, CharOrEol, CharPos, EndOfLine, LazyCharGrid, LnCol,
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
fn lazy_char_at_char_pos() {
    let grid = partially_loaded_grid();

    eprintln!("TEST (0)");
    let char = grid
        .char_at(CharPos::from_pred_count(0))
        .expect("char_at (0)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 0);
    assert_eq!(char.value().to_string(), "H");
    assert!(matches!(char.value(), CharOrEol::Char('H')));
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid
    assert_eq!(grid.loaded_char_count(), 7); // preloaded from partially_loaded_grid

    eprintln!("TEST (4)");
    let char = grid
        .char_at(CharPos::from_pred_count(4))
        .expect("char_at (4)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 4));
    assert_eq!(char.pos(), CharPos::from_pred_count(4));
    assert_eq!(char.offset_from_ln_start(), 4);
    assert_eq!(char.offset_from_doc_start(), "Hell".len());
    assert_eq!(char.value().to_string(), "o");
    assert!(matches!(char.value(), CharOrEol::Char('o')));
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤"); // preloaded from partially_loaded_grid
    assert_eq!(grid.loaded_char_count(), 7); // preloaded from partially_loaded_grid

    eprintln!("TEST (6)");
    let char = grid
        .char_at(CharPos::from_pred_count(6))
        .expect("char_at (6)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 6));
    assert_eq!(char.pos(), CharPos::from_pred_count(6));
    assert_eq!(char.offset_from_ln_start(), 6);
    assert_eq!(char.offset_from_doc_start(), "Hello,".len());
    assert_eq!(char.value().to_string(), "\n");
    assert!(matches!(char.value(), CharOrEol::EndOfLine(EndOfLine::LF)));
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤");
    assert_eq!(grid.loaded_char_count(), 7);

    eprintln!("TEST (6 + 1)");
    let char = grid
        .char_at(CharPos::from_pred_count(6 + 1))
        .expect("char_at (6 + 1)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), "Hello,\n".len());
    assert_eq!(char.value().to_string(), "I");
    assert!(matches!(char.value(), CharOrEol::Char('I')));
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 20);

    eprintln!("TEST (6 + 1 + 2)");
    let char = grid
        .char_at(CharPos::from_pred_count(6 + 1 + 2))
        .expect("char_at (6 + 1 + 2)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 2));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1 + 2));
    assert_eq!(char.offset_from_ln_start(), 2);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ".len());
    assert_eq!(char.value().to_string(), "❤");
    assert!(matches!(char.value(), CharOrEol::Char('❤')));
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 20);

    eprintln!("TEST (6 + 1 + 12)");
    let char = grid
        .char_at(CharPos::from_pred_count(6 + 1 + 12))
        .expect("char_at (6 + 1 + 12)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 12));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1 + 12));
    assert_eq!(char.offset_from_ln_start(), 19);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ❤️ Rust 🦀,".len());
    assert_eq!(char.value().to_string(), "\r\n");
    assert!(matches!(
        char.value(),
        CharOrEol::EndOfLine(EndOfLine::CRLF),
    ));
    assert_eq!(grid.data().loaded_text(), "Hello,\nI ❤️ Rust 🦀,\r\n");
    assert_eq!(grid.loaded_char_count(), 20);

    eprintln!("TEST (6 + 1 + 12 + 1 + 45 + 1)");
    let char = grid
        .char_at(CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1))
        .expect("char_at (6 + 1 + 12 + 1 + 45 + 1)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(3, 0));
    assert_eq!(
        char.pos(),
        CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1),
    );
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 74);
    assert_eq!(char.value().to_string(), "T");
    assert!(matches!(char.value(), CharOrEol::Char('T')));
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST (6 + 1 + 12 + 1 + 45 + 1 + 35)");
    let char = grid
        .char_at(CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1 + 35))
        .expect("char_at (6 + 1 + 12 + 1 + 45 + 1 + 35)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(3, 35));
    assert_eq!(
        char.pos(),
        CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1 + 35),
    );
    assert_eq!(char.offset_from_ln_start(), 35);
    assert_eq!(char.offset_from_doc_start(), 109);
    assert_eq!(char.value().to_string(), "🥚");
    assert!(matches!(char.value(), CharOrEol::Char('🥚')));
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST (6 + 1 + 12 + 1 + 45 + 1 + 35 + 1)");
    let char = grid
        .char_at(CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1 + 35 + 1))
        .expect("char_at (6 + 1 + 12 + 1 + 45 + 1 + 35 + 1)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(3, 36));
    assert_eq!(
        char.pos(),
        CharPos::from_pred_count(6 + 1 + 12 + 1 + 45 + 1 + 35 + 1),
    );
    assert_eq!(char.offset_from_ln_start(), 39);
    assert_eq!(char.offset_from_doc_start(), SRC_TEXT.len());
    assert_eq!(char.value().to_string(), "");
    assert!(matches!(char.value(), CharOrEol::EndOfLine(EndOfLine::EOF)));
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST (6 + 1 + 12 + 1 + 45 + 1 + 35 + 1 + 1)");
    let error = grid
        .char_at(CharPos::from_pred_count(
            6 + 1 + 12 + 1 + 45 + 1 + 35 + 1 + 1,
        ))
        .expect_err("char_at (6 + 1 + 12 + 1 + 45 + 1 + 35 + 1 + 1)");
    assert_eq!(error, lazy_char_grid::CharAtCharPosError::OutOfBound);

    eprintln!("TEST (0) (again)");
    let char = grid
        .char_at(CharPos::from_pred_count(0))
        .expect("char_at (0)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(0, 0));
    assert_eq!(char.pos(), CharPos::from_pred_count(0));
    assert_eq!(char.offset_from_ln_start(), 0);
    assert_eq!(char.offset_from_doc_start(), 0);
    assert_eq!(char.value().to_string(), "H");
    assert!(matches!(char.value(), CharOrEol::Char('H')));
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());

    eprintln!("TEST (6 + 1 + 2) (again)");
    let char = grid
        .char_at(CharPos::from_pred_count(6 + 1 + 2))
        .expect("char_at (6 + 1 + 2)");
    assert_eq!(char.coord(), LnCol::from_pred_counts(1, 2));
    assert_eq!(char.pos(), CharPos::from_pred_count(6 + 1 + 2));
    assert_eq!(char.offset_from_ln_start(), 2);
    assert_eq!(char.offset_from_doc_start(), "Hello,\nI ".len());
    assert_eq!(char.value().to_string(), "❤");
    assert!(matches!(char.value(), CharOrEol::Char('❤')));
    assert_eq!(grid.data().loaded_text(), SRC_TEXT);
    assert_eq!(grid.loaded_char_count(), SRC_TEXT.chars().count());
}
