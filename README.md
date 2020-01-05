terminal-size
=============


[Documention](https://eminence.github.io/terminal-size/doc/terminal_size/index.html)


Rust library to getting the size of your terminal.

Works on Linux and Windows, but needs testing on other platforms

Tested on Rust Stable (1.8), Beta (1.9), and Nightly (1.10)

```rust
use terminal_size::{Width, Height, terminal_size};

let size = terminal_size();
if let Some((Width(w), Height(h))) = size {
    println!("Your terminal is {} cols wide and {} lines tall", w, h);
} else {
    println!("Unable to get terminal size");
}
```

## Minimum Rust Version

This crate requires a minimum rust version of 1.31.0 (2018-12-06)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
