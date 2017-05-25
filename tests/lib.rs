extern crate pbpaste;

use std::str::from_utf8;

fn set_and_get_text(s: &'static str, strip_cr: bool) -> Option<String> {
    let mut clipboard = pbpaste::Clipboard::new();

    clipboard.set_text(s);
    clipboard.get_text(strip_cr).map(|text| {
        String::from(from_utf8(text.as_ref()).unwrap())
    })
}

#[test]
fn get_clipboard_text_works_when_clipboard_has_text() {
    assert_eq!(set_and_get_text("hello there", false).unwrap(),
               "hello there");
}

#[test]
fn get_clipboard_text_ignores_unprintable_characters() {
    assert_eq!(set_and_get_text("how\x07 goes", false).unwrap(),
               "how goes");
}

#[test]
fn get_clipboard_text_ignores_non_ascii_characters() {
    assert_eq!(set_and_get_text("how\u{2026} goes", false).unwrap(),
               "how goes");
}

#[test]
fn get_clipboard_text_does_not_strip_cr() {
    assert_eq!(set_and_get_text("hello there\r\n", false).unwrap(),
               "hello there\r\n");
}

#[test]
fn get_clipboard_text_strips_cr() {
    assert_eq!(set_and_get_text("hello there\r\n", true).unwrap(),
               "hello there\n");
}

#[test]
fn get_clipboard_text_is_none_when_it_has_no_valid_chars() {
    assert!(set_and_get_text("\u{2026}", false).is_none());
}

#[test]
fn get_clipboard_text_works_when_clipboard_is_empty() {
    let mut clipboard = pbpaste::Clipboard::new();

    clipboard.empty();
    assert!(clipboard.get_text(false).is_none());
}
