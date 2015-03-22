#![feature(io)]
#![feature(core)]
#![feature(exit_status)]
#![feature(libc)]
#![allow(dead_code)]

extern crate "user32-sys" as user32;
extern crate winapi;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard};
use std::ffi::CStr;
use std::io::Write;
use std::io::stdout;
use std::env;

mod windows_clipboard_types {
  pub static CF_TEXT: u32 = 1;
}

static USAGE: &'static str = "\
Output plain-text clipboard content.

Usage:
  pbpaste [--dos|--unix]

Options:
  -h --help    Show this screen.
  --dos        Output DOS (CR+LF) line endings.
  --unix       Output Unix (LF) line endings (default).
";

fn help(exit_code: i32) {
    println!("{}", USAGE);
    std::env::set_exit_status(exit_code);
}

fn get_clipboard_text(strip_cr: bool) -> Option<Vec<u8>> {
    let slice_bytes: &[u8];

    unsafe {
        let success = OpenClipboard(0 as winapi::HWND);
        if success == 0 {
            panic!("OpenClipboard() failed!");
        }
        let data = GetClipboardData(windows_clipboard_types::CF_TEXT);
        if data.is_null() {
            CloseClipboard();
            return None;
        }
        slice_bytes = CStr::from_ptr(data as *const i8).to_bytes();
    }

    let mut clipboard: Vec<u8> = Vec::with_capacity(slice_bytes.len());

    for i in slice_bytes {
        if strip_cr {
            if *i != 13 {
                clipboard.push(*i);
            }
        } else {
            clipboard.push(*i);
        }
    }

    unsafe {
        CloseClipboard();
    }

    Some(clipboard)
}

fn main() {
    let strip_cr: bool;
    let args: Vec<String> = env::args().collect();

    match &args[..] {
        [_] => {
            strip_cr = true;
        },
        [_, ref option] => {
            match &option[..] {
                "-h" | "--help" => { return help(0); },
                "--dos" => { strip_cr = false; }
                "--unix" => { strip_cr = true; }
                _ => {
                    return help(1);
                }
            }
        },
        _ => { return help(1); }
    }

    match get_clipboard_text(strip_cr) {
        Some(clipboard_text) => {
            match stdout().write_all(clipboard_text.as_slice()) {
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
        },
        None => {}
    }
}

mod windows_gmem_types {
    pub static GMEM_MOVEABLE: u32 = 0x0002;
}

#[link(name = "kernel32")]
extern "system" {
    fn GlobalAlloc(uFlags: winapi::UINT,
                   dwBytes: winapi::SIZE_T) -> winapi::HGLOBAL;

    fn GlobalLock(hMem: winapi::HGLOBAL) -> winapi::LPVOID;
    fn GlobalUnlock(hMem: winapi::HGLOBAL) -> winapi::BOOL;
}

#[link(name = "user32")]
extern "system" {
    fn SetClipboardData(uFormat: winapi::UINT,
                        hMem: winapi::HANDLE) -> winapi::HANDLE;
}


extern crate libc;
use libc::strcpy;
use std::ffi::CString;

#[test]
fn it_works() {
    fn set_clipboard_text(test_str: &str) {
        let test_cstring = CString::new(test_str).unwrap();

        unsafe {
            if OpenClipboard(0 as winapi::HWND) == 0 {
                panic!("OpenClipboard() failed!");
            }
            user32::EmptyClipboard();

            let copy = GlobalAlloc(windows_gmem_types::GMEM_MOVEABLE,
                                   (test_str.len() + 1) as winapi::SIZE_T);
            if copy.is_null() {
                panic!("GlobalAlloc() failed!");
            }
            let str_copy = GlobalLock(copy);
            if str_copy.is_null() {
                panic!("GlobalLock() failed!");
            }
            strcpy(str_copy as *mut i8, test_cstring.as_ptr());
            GlobalUnlock(copy);
            if SetClipboardData(windows_clipboard_types::CF_TEXT,
                                copy).is_null() {
                panic!("SetClipboardData() failed!");
            }
            CloseClipboard();
        }
    }

    set_clipboard_text("hello there");

    match get_clipboard_text(false) {
        Some(vec) => {
            let utf8 = std::str::from_utf8(vec.as_slice()).unwrap();
            assert_eq!(utf8, "hello there");
        },
        None => {
            panic!("expected clipboard to contain text");
        }
    }
}
