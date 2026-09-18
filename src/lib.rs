//! A simple utility for getting the size of a terminal.
//!
//! Works on Linux, macOS, Windows, and illumos.
//!
//! This crate requires a minimum Rust version of 1.71.0 (2023-07-13).
//!
//! # Example
//!
//! ```
//! use terminal_size::{Width, Height, terminal_size};
//!
//! let size = terminal_size();
//! if let Some((Width(w), Height(h))) = size {
//!     println!("Your terminal is {} cols wide and {} lines tall", w, h);
//! } else {
//!     println!("Unable to get terminal size");
//! }
//! ```
//!

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Width(pub u16);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Height(pub u16);

mod sys;

#[cfg(any(
    unix,
    target_os = "hermit",
    target_os = "motor",
    target_os = "trusty",
    target_os = "wasi"
))]
#[allow(deprecated)]
pub use crate::sys::fd::{terminal_size, terminal_size_of, terminal_size_using_fd};

#[cfg(windows)]
#[allow(deprecated)]
pub use crate::sys::windows::{terminal_size, terminal_size_of, terminal_size_using_handle};

#[cfg(all(
    feature = "unsupported",
    not(any(
        unix,
        windows,
        target_os = "hermit",
        target_os = "motor",
        target_os = "trusty",
        target_os = "wasi"
    ))
))]
pub use crate::sys::unsupported::terminal_size;
