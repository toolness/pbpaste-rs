extern crate user32;
extern crate kernel32;
extern crate winapi;
extern crate libc;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard, EmptyClipboard,
             SetClipboardData};
use kernel32::{GlobalAlloc, GlobalLock, GlobalUnlock};
use libc::{memcpy, c_void};
use std::ffi::CString;
use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

enum Ascii {
    CarriageReturn = 13,
    LineFeed = 10,
    Tab = 9,
    Space = 32,
    Squiggle = 126,
}

pub enum Linefeeds {
    Dos,
    Unix
}

static GLOBAL_CLIPBOARD_LOCK: AtomicBool = ATOMIC_BOOL_INIT;

pub mod windows_clipboard_types {
  pub static CF_TEXT: u32 = 1;
}

mod windows_gmem_types {
    pub static GMEM_MOVEABLE: u32 = 0x0002;
}

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Clipboard {
        while GLOBAL_CLIPBOARD_LOCK.compare_and_swap(
            false,
            true,
            Ordering::Relaxed
        ) {}
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
            memcpy(str_copy as *mut c_void,
                   test_cstring.as_ptr() as *const c_void,
                   test_str.len() + 1);
            GlobalUnlock(copy);
            if SetClipboardData(windows_clipboard_types::CF_TEXT,
                                copy).is_null() {
                panic!("SetClipboardData() failed!");
            }
        }
        self.close();
    }

    pub fn get_text(&self, linefeeds: Linefeeds) -> Option<Vec<u8>> {
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

        for &i in slice_bytes {
            let mut push = false;

            if i == Ascii::CarriageReturn as u8 {
                push = match linefeeds {
                    Linefeeds::Dos => true,
                    Linefeeds::Unix => false,
                };
            } else if i == Ascii::LineFeed as u8 ||
                      i == Ascii::Tab as u8 ||
                      (i >= Ascii::Space as u8 &&
                       i <= Ascii::Squiggle as u8) {
                push = true;
            }

            if push {
                clipboard.push(i);
            }
        }

        self.close();

        if clipboard.len() > 0 { Some(clipboard) } else { None }
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        GLOBAL_CLIPBOARD_LOCK.store(false, Ordering::Relaxed);
    }
}
