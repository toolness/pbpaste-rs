#![feature(libc)]

extern crate libc;
extern crate "user32-sys" as user32;
extern crate winapi;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard, EmptyClipboard};
use libc::strcpy;
use std::ffi::CString;
use std::ffi::CStr;

pub mod windows_clipboard_types {
  pub static CF_TEXT: u32 = 1;
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

pub fn get_clipboard_text(strip_cr: bool) -> Option<Vec<u8>> {
    let slice_bytes: &[u8];

    unsafe {
        if OpenClipboard(0 as winapi::HWND) == 0 {
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

pub fn empty_clipboard() {
    unsafe {
        if OpenClipboard(0 as winapi::HWND) == 0 {
            panic!("OpenClipboard() failed!");
        }
        EmptyClipboard();
        CloseClipboard();
    }
}

pub fn set_clipboard_text(test_str: &str) {
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
        if SetClipboardData(windows_clipboard_types::CF_TEXT,
                            copy).is_null() {
            panic!("SetClipboardData() failed!");
        }
        CloseClipboard();
    }
}
