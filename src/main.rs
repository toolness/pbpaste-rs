#![feature(io)]
#![feature(core)]
#![feature(exit_status)]

extern crate pbpaste;

use std::io::Write;
use std::io::stdout;
use std::env;

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

// Note that we have the [allow(dead_code)] attribute below because
// the test suite complains about it (presumably because
// the function is not being tested).

#[allow(dead_code)]
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

    let clipboard = pbpaste::Clipboard::new();

    match clipboard.get_text(strip_cr) {
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
