#![feature(core)]

extern crate pbpaste;

use std::str::from_utf8;

fn get_clipboard_text_works_when_clipboard_has_text() {
    pbpaste::set_clipboard_text("hello there");
    let clip_text = pbpaste::get_clipboard_text(false).unwrap();
    assert_eq!(from_utf8(clip_text.as_slice()).unwrap(), "hello there");
}

fn get_clipboard_text_works_when_clipboard_is_empty() {
    pbpaste::empty_clipboard();
    let clip_text = pbpaste::get_clipboard_text(false);
    match clip_text {
        None => {
        },
        _ => {
            panic!("Expected no clipboard text!");
        }
    }
}

// We can't run our tests in parallel because we're using the
// OS's global clipboard, so we'll have to use this hack to
// do it serially.
//
// For more information, see:
// https://github.com/rust-lang/rust/issues/1813#issuecomment-20911468

#[test]
fn all_tests_in_serial() {
    get_clipboard_text_works_when_clipboard_has_text();
    get_clipboard_text_works_when_clipboard_is_empty();
}
