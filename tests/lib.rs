extern crate pbpaste;

use pbpaste::{Clipboard, Newlines};
use pbpaste::Newlines::*;

fn set_and_get_text(s: &'static str, newlines: Newlines) -> String {
    let mut clipboard = Clipboard::new();

    clipboard.set_ascii_text(s);
    clipboard.get_text(newlines)
}

#[test]
fn get_clipboard_text_works_when_clipboard_has_text() {
    assert_eq!(set_and_get_text("hello there", Dos),
               "hello there");
}

#[test]
fn get_clipboard_text_includes_weird_characters() {
    assert_eq!(set_and_get_text("how\x07 goes", Dos),
               "how\u{7} goes");
}

#[test]
fn get_clipboard_text_does_not_strip_cr() {
    assert_eq!(set_and_get_text("hello there\r\n", Dos),
               "hello there\r\n");
}

#[test]
fn get_clipboard_text_strips_cr() {
    assert_eq!(set_and_get_text("hello there\r\n", Unix),
               "hello there\n");
}

#[test]
fn get_clipboard_text_works_with_empty_string() {
    assert_eq!(set_and_get_text("", Unix), "");
}

#[test]
fn get_clipboard_text_works_when_clipboard_is_empty() {
    let mut clipboard = Clipboard::new();

    clipboard.empty();
    assert_eq!(clipboard.get_text(Dos), "");
}
