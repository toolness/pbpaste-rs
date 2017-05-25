This is a simple implementation of OS X's `pbpaste` utility in Rust
for Windows systems.

Because it's my first Rust program, it is probably full of horrible
things.

To compile it, just run `cargo build`. The executable will be in
`target\debug\pbpaste.exe`.

## Usage

```
Output plain-text clipboard content.

Usage:
  pbpaste [--dos|--unix]

Options:
  -h --help    Show this screen.
  --dos        Output DOS (CR+LF) line endings.
  --unix       Output Unix (LF) line endings (default).
```

## Notes

  * There is no analogous `pbcopy` tool because Windows already
    comes with one called `clip.exe`.
