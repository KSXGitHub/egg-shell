use egg_text::{text_slice::ScanText, CharCell, CharCoord};
use pretty_assertions::assert_eq;

#[test]
fn char_offset() {
    let src_text = "I Love ❤️ Rust 🦀!";
    let mut char_list = Vec::<CharCell<char>>::new();
    ScanText::run(ScanText {
        char_list: &mut char_list,
        src_text,
        first_char_coord: CharCoord::from_pred_counts(0, 0),
        offset: 0,
    });
    let mut received = Vec::new();
    for char_cell in char_list.iter().copied() {
        dbg!(char_cell);
        let offset = char_cell.offset_from_ln_start();
        dbg!(offset);
        let before = &src_text[..offset];
        dbg!(before);
        let after = &src_text[offset..];
        dbg!(after);
        received.push((before, after));
    }
    dbg!(&received);
    let expected = [
        ("", "I Love ❤️ Rust 🦀!"),
        ("I", " Love ❤️ Rust 🦀!"),
        ("I ", "Love ❤️ Rust 🦀!"),
        ("I L", "ove ❤️ Rust 🦀!"),
        ("I Lo", "ve ❤️ Rust 🦀!"),
        ("I Lov", "e ❤️ Rust 🦀!"),
        ("I Love", " ❤️ Rust 🦀!"),
        ("I Love ", "❤️ Rust 🦀!"),
        ("I Love ❤", "\u{fe0f} Rust 🦀!"),
        ("I Love ❤️", " Rust 🦀!"),
        ("I Love ❤️ ", "Rust 🦀!"),
        ("I Love ❤️ R", "ust 🦀!"),
        ("I Love ❤️ Ru", "st 🦀!"),
        ("I Love ❤️ Rus", "t 🦀!"),
        ("I Love ❤️ Rust", " 🦀!"),
        ("I Love ❤️ Rust ", "🦀!"),
        ("I Love ❤️ Rust 🦀", "!"),
    ];
    assert_eq!(received, expected);
}
