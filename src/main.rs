#![feature(io)]
#![feature(core)]

extern crate "user32-sys" as user32;
extern crate winapi;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard};
use std::ffi::CStr;
use std::io::Write;
use std::io::stdout;

fn main() {
    let mut clipboard: Vec<u8> = Vec::new();
    let strip_linefeeds = true;

    unsafe {
        let success = OpenClipboard(0 as winapi::HWND);
        if success == 0 {
            panic!("OpenClipboard() failed!");
        }
        let data = GetClipboardData(1); // 1 is CF_TEXT
        if data.is_null() {
            CloseClipboard();
            return;
        }
        let slice_bytes = CStr::from_ptr(data as *const i8).to_bytes();

        for i in slice_bytes {
            if strip_linefeeds {
                if *i != 10 {
                    clipboard.push(*i);
                }
            } else {
                clipboard.push(*i);
            }
        }

        CloseClipboard();
    }

    match stdout().write_all(clipboard.as_slice()) {
        Ok(_) => {
            match stdout().flush() {
                Ok(_) => {
                },
                Err(_) => {
                    panic!("flush() failed!");
                }
            }
        },
        Err(_) => {
            panic!("write_all() failed!");
        }
    }
}
