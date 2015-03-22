#![feature(libc)]
#![feature(core)]

extern crate pbpaste;
extern crate libc;
extern crate "user32-sys" as user32;
extern crate winapi;

use libc::strcpy;
use std::ffi::CString;
use std::str::from_utf8;
use user32::{OpenClipboard, CloseClipboard, EmptyClipboard};

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

fn set_clipboard_text(test_str: &str) {
    let test_cstring = CString::new(test_str).unwrap();

    unsafe {
        if OpenClipboard(0 as winapi::HWND) == 0 {
            panic!("OpenClipboard() failed!");
        }
        EmptyClipboard();

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
        if SetClipboardData(pbpaste::windows_clipboard_types::CF_TEXT,
                            copy).is_null() {
            panic!("SetClipboardData() failed!");
        }
        CloseClipboard();
    }
}

#[test]
fn get_clipboard_text_works() {
    set_clipboard_text("hello there");
    let clip_text = pbpaste::get_clipboard_text(false).unwrap();
    assert_eq!(from_utf8(clip_text.as_slice()).unwrap(), "hello there");
}