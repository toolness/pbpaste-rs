extern crate "user32-sys" as user32;
extern crate winapi;

use user32::{OpenClipboard, GetClipboardData, CloseClipboard};
use std::ffi::CStr;

pub mod windows_clipboard_types {
  pub static CF_TEXT: u32 = 1;
}

pub fn get_clipboard_text(strip_cr: bool) -> Option<Vec<u8>> {
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
