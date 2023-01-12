use crate::{
    char_grid::lazy_char_grid, CharAt, LazyCharGrid, LnCol, LnNum, SliceFrom, TryIterChar,
    TryIterLine,
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
fn lazy_slice_from_ln_num_try_iter_char() {
    let grid = partially_loaded_grid();

    eprintln!("create the slice");
    let slice = grid
        .slice_from(LnNum::from_pred_count(1))
        .expect("slice ln 2")
        .slice_from(LnNum::from_pred_count(1))
        .expect("slice ln 2");

    let mut acc = String::new();
    for char_result in slice.try_iter_char() {
        let char = char_result.expect("get char");
        let pos = char.pos();
        let string = char.to_string();
        eprintln!("char = {char:?}; pos = {pos:?}");
        assert_eq!(
            grid.char_at(char.coord())
                .expect("grid.char_at ln_col")
                .to_string(),
            string,
        );
        assert_eq!(
            grid.char_at(pos).expect("grid.char_at pos").to_string(),
            string,
        );
        acc += &string;
    }

    eprintln!("ACCUMULATION:\n{acc}\n");
    let offset = slice
        .char_at(LnCol::from_pred_counts(0, 0))
        .expect("char_at 1:1")
        .offset_from_doc_start;
    assert_eq!(acc, SRC_TEXT[offset..]);
    assert_eq!(
        acc,
        concat! {
            "So I use it to create a programming language,\n",
            "The language is called 'egg-shell' 🥚",
        },
    );
}

#[test]
fn lazy_slice_from_ln_num_try_iter_line() {
    let grid = partially_loaded_grid();

    eprintln!("create the slice");
    let slice = grid
        .slice_from(LnNum::from_pred_count(1))
        .expect("slice ln 2")
        .slice_from(LnNum::from_pred_count(1))
        .expect("slice ln 2");

    let mut acc = String::new();
    for line_result in slice.try_iter_line() {
        let line = dbg!(line_result.expect("get line"));
        acc += line.to_string().as_str();
    }

    eprintln!("ACCUMULATION:\n{acc}\n");
    let offset = slice
        .char_at(LnCol::from_pred_counts(0, 0))
        .expect("char_at 1:1")
        .offset_from_doc_start;
    assert_eq!(acc, SRC_TEXT[offset..]);
    assert_eq!(
        acc,
        concat! {
            "So I use it to create a programming language,\n",
            "The language is called 'egg-shell' 🥚",
        },
    );
}
