#![feature(io)]
#![feature(core)]
#![feature(exit_status)]

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

fn help(exit_code: i32) {
    println!("\
Output plain-text clipboard content.

Usage:
  pbpaste [--dos|--unix]

Options:
  -h --help    Show this screen.
  --dos        Output DOS (CR+LF) line endings.
  --unix       Output Unix (LF) line endings (default).
");
    std::env::set_exit_status(exit_code);
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
                "-h" => { return help(0); },
                "--help" => { return help(0); },
                "--dos" => { strip_cr = false; }
                "--unix" => { strip_cr = true; }
                _ => {
                    return help(1);
                }
            }
        },
        _ => { return help(1); }
    }

    let slice_bytes: &[u8];

    unsafe {
        let success = OpenClipboard(0 as winapi::HWND);
        if success == 0 {
            panic!("OpenClipboard() failed!");
        }
        let data = GetClipboardData(windows_clipboard_types::CF_TEXT);
        if data.is_null() {
            CloseClipboard();
            return;
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
