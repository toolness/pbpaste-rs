This is a simple implementation of OS X's `pbpaste` utility in Rust
for Windows systems.

Because it's my first Rust program, it is probably full of horrible
things.

To compile it, just run `cargo build`. The executable will be in
`target\debug\pbpaste.exe`.

Notes:

  * It doesn't currently support unicode.
  * It currently strips linefeeds out, so that the resulting output
    contains UNIX line endings instead of Windows/DOS line endings.
  * There is no analogous `pbcopy` tool because Windows already
    comes with one called `clip.exe`.
