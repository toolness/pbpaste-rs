extern crate pbpaste;

use std::str::from_utf8;

#[test]
fn get_clipboard_text_works_when_clipboard_has_text() {
    let mut clipboard = pbpaste::Clipboard::new();

    clipboard.set_text("hello there");
    let clip_text = clipboard.get_text(false).unwrap();
    assert_eq!(from_utf8(clip_text.as_ref()).unwrap(), "hello there");
}

#[test]
fn get_clipboard_text_does_not_strip_cr() {
    let mut clipboard = pbpaste::Clipboard::new();

    clipboard.set_text("hello there\r\n");
    let clip_text = clipboard.get_text(false).unwrap();
    assert_eq!(from_utf8(clip_text.as_ref()).unwrap(), "hello there\r\n");
}

#[test]
fn get_clipboard_text_strips_cr() {
    let mut clipboard = pbpaste::Clipboard::new();

    clipboard.set_text("hello there\r\n");
    let clip_text = clipboard.get_text(true).unwrap();
    assert_eq!(from_utf8(clip_text.as_ref()).unwrap(), "hello there\n");
}

#[test]
fn get_clipboard_text_works_when_clipboard_is_empty() {
    let mut clipboard = pbpaste::Clipboard::new();

    clipboard.empty();
    let clip_text = clipboard.get_text(false);
    match clip_text {
        None => {
        },
        _ => {
            panic!("Expected no clipboard text!");
        }
    }
}
