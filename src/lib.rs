extern crate user32;
extern crate kernel32;
extern crate winapi;
extern crate libc;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard, EmptyClipboard,
             SetClipboardData};
use kernel32::{GlobalAlloc, GlobalLock, GlobalUnlock, WideCharToMultiByte};
use winapi::winnt::{WCHAR};
use libc::{memcpy, c_void};
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use self::windows_clipboard_types::*;

#[derive(PartialEq)]
pub enum Linefeeds {
    Dos,
    Unix
}

static GLOBAL_CLIPBOARD_LOCK: AtomicBool = ATOMIC_BOOL_INIT;

pub mod windows_clipboard_types {
  pub static CF_TEXT: u32 = 1;
  pub static CF_UNICODETEXT: u32 = 13;
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

    pub fn set_ascii_text(&mut self, test_str: &str) {
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
            if SetClipboardData(CF_TEXT, copy).is_null() {
                panic!("SetClipboardData() failed!");
            }
        }
        self.close();
    }

    pub fn get_text(&self, linefeeds: Linefeeds) -> Option<Vec<u8>> {
        let mut slice_bytes: Vec<u8>;
        let bytes_required;

        self.open();
        unsafe {
            let data = GetClipboardData(CF_UNICODETEXT);
            if data.is_null() {
                self.close();
                return None;
            }
            bytes_required = WideCharToMultiByte(
                winapi::winnls::CP_UTF8,                // CodePage
                0,                                      // dwFlags
                data as *const WCHAR,                   // lpWideCharStr
                -1,                                     // cchWideChar
                0 as *mut i8,                           // lpMultiByteStr
                0,                                      // cbMultiByte
                0 as *const i8,                         // lpDefaultChar
                0 as *mut i32,                          // lpUsedDefaultChar
            );

            slice_bytes = vec![0; bytes_required as usize];

            let result = WideCharToMultiByte(
                winapi::winnls::CP_UTF8,                // CodePage
                0,                                      // dwFlags
                data as *const WCHAR,                   // lpWideCharStr
                -1,                                     // cchWideChar
                slice_bytes.as_mut_ptr() as *mut i8,    // lpMultiByteStr
                bytes_required,                         // cbMultiByte
                0 as *const i8,                         // lpDefaultChar
                0 as *mut i32,                          // lpUsedDefaultChar
            );

            if result == 0 {
                panic!("WideCharToMultiByte() failed!");
            }

            slice_bytes.set_len((bytes_required - 1) as usize);
        }

        if linefeeds == Linefeeds::Unix {
            // TODO: Strip CRs.
        }
        self.close();

        Some(slice_bytes)
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        GLOBAL_CLIPBOARD_LOCK.store(false, Ordering::Relaxed);
    }
}
