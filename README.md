terminal-size
=============


[Documention](https://eminence.github.io/terminal-size/doc/terminal_size/index.html)


Rust library to getting the size of your terminal.

Works on Linux and Windows, but needs testing on other platforms

```rust
use terminal_size::{Width, Height, terminal_size};

let size = terminal_size();
if let Some((Width(w), Height(h))) = size {
    println!("Your terminal is {} cols wide and {} lines tall", w, h);
} else {
    println!("Unable to get terminal size");
}
```
