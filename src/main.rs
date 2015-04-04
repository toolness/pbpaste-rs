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
    if exit_code != 0 {
        println!("Panicking because feature(exit_status) isn't available.");
        panic!();
    }
}

// Note that we have the [allow(dead_code)] attribute below because
// the test suite complains about it (presumably because
// the function is not being tested).

#[allow(dead_code)]
fn main() {
    let strip_cr: bool;
    let args: Vec<String> = env::args().collect();

    // We used to use pattern matching for this, but feature(slice_patterns) isn't
    // supported in Rust 1.0.0 Beta.
    if args.len() == 1 {
        strip_cr = true;
    } else if args.len() == 2 {
        match &args[1][..] {
            "-h" | "--help" => { return help(0); },
            "--dos" => { strip_cr = false; }
            "--unix" => { strip_cr = true; }
            _ => {
                return help(1);
            }
        }
    } else {
        return help(1);
    }

    let clipboard = pbpaste::Clipboard::new();

    match clipboard.get_text(strip_cr) {
        Some(clipboard_text) => {
            match stdout().write_all(clipboard_text.as_ref()) {
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
