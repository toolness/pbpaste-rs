extern crate user32;
extern crate kernel32;
extern crate winapi;
extern crate libc;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard, EmptyClipboard,
             SetClipboardData};
use kernel32::{GlobalAlloc, GlobalLock, GlobalUnlock, WideCharToMultiByte};
use winapi::winnt::{WCHAR};
use libc::{memcpy, c_void};
use std::mem::drop;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

use self::windows_clipboard_types::*;

pub enum Newlines {
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

struct ClipboardOpener;

impl ClipboardOpener {
    fn new() -> Self {
        unsafe {
            if OpenClipboard(0 as winapi::HWND) == 0 {
                panic!("OpenClipboard() failed!");
            }
        }
        ClipboardOpener
    }
}

impl Drop for ClipboardOpener {
    fn drop(&mut self) {
        unsafe {
            CloseClipboard();
        }
    }
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

    pub fn empty(&mut self) {
        let opener = ClipboardOpener::new();
        unsafe {
            EmptyClipboard();
        }
        drop(opener);
    }

    pub fn set_ascii_text(&mut self, test_str: &str) {
        let test_cstring = CString::new(test_str).unwrap();

        let opener = ClipboardOpener::new();
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
        drop(opener);
    }

    pub fn get_text(&self, newlines: Newlines) -> String {
        let mut slice_bytes: Vec<u8>;
        let bytes_required;

        let opener = ClipboardOpener::new();
        unsafe {
            let data = GetClipboardData(CF_UNICODETEXT);
            if data.is_null() {
                return String::from("");
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
        drop(opener);

        let result = String::from_utf8(slice_bytes).unwrap();

        match newlines {
            Newlines::Unix => { strip_crs(result) },
            Newlines::Dos => result,
        }
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        GLOBAL_CLIPBOARD_LOCK.store(false, Ordering::Relaxed);
    }
}

fn strip_crs<T: AsRef<str>>(s: T) -> String {
    let mut result = String::with_capacity(s.as_ref().len());

    for c in s.as_ref().chars() {
        if c != '\r' {
            result.push(c);
        }
    }

    result
}
