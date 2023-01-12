use crate::{
    char_grid::completed_char_grid, CharAt, CharPos, CompletedCharGrid, IterChar, LazyCharGrid,
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
