#![feature(libc)]

extern crate libc;
extern crate user32_sys as user32;
extern crate winapi;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard, EmptyClipboard};
use libc::strcpy;
use std::ffi::CString;
use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

static GLOBAL_CLIPBOARD_LOCK: AtomicBool = ATOMIC_BOOL_INIT;

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

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Clipboard {
        while GLOBAL_CLIPBOARD_LOCK.compare_and_swap(false, true, Ordering::Relaxed) {}
        Clipboard
    }

    fn open(&self) {
        unsafe {
            if OpenClipboard(0 as winapi::HWND) == 0 {
                panic!("OpenClipboard() failed!");
            }
        }
    }

    fn close(&self) {
        unsafe {
            CloseClipboard();
        }
    }

    pub fn empty(&mut self) {
        self.open();
        unsafe {
            EmptyClipboard();
        }
        self.close();
    }

    pub fn set_text(&mut self, test_str: &str) {
        let test_cstring = CString::new(test_str).unwrap();

        self.open();
        unsafe {
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
        }
        self.close();
    }

    pub fn get_text(&self, strip_cr: bool) -> Option<Vec<u8>> {
        let slice_bytes: &[u8];

        self.open();
        unsafe {
            let data = GetClipboardData(windows_clipboard_types::CF_TEXT);
            if data.is_null() {
                self.close();
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

        self.close();

        Some(clipboard)
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        GLOBAL_CLIPBOARD_LOCK.store(false, Ordering::Relaxed);
    }
}
