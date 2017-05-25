extern crate pbpaste;

use std::io::Write;
use std::io::stdout;
use std::env;
use pbpaste::{Clipboard, Newlines};

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
    std::process::exit(exit_code);
}

fn main() {
    let newlines: Newlines;
    let args: Vec<String> = env::args().collect();

    // We used to use pattern matching for this, but feature(slice_patterns) isn't
    // supported in Rust 1.0.0 Beta.
    if args.len() == 1 {
        newlines = Newlines::Unix;
    } else if args.len() == 2 {
        match &args[1][..] {
            "-h" | "--help" => { return help(0); },
            "--dos" => { newlines = Newlines::Dos; }
            "--unix" => { newlines = Newlines::Unix; }
            _ => {
                return help(1);
            }
        }
    } else {
        return help(1);
    }

    let clipboard = Clipboard::new();

    let text = clipboard.get_text(newlines);
    match stdout().write_all(text.as_str().as_ref()) {
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
