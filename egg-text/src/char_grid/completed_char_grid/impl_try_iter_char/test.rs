use crate::{CharPos, CompletedCharGrid, IterChar, LazyCharGrid};
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
