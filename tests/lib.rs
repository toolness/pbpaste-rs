#![feature(core)]

extern crate pbpaste;

use std::str::from_utf8;

mod util;

#[test]
fn get_clipboard_text_works() {
    util::set_clipboard_text("hello there");
    let clip_text = pbpaste::get_clipboard_text(false).unwrap();
    assert_eq!(from_utf8(clip_text.as_slice()).unwrap(), "hello there");
}
