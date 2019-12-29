//! A simple utility for getting the size of a terminal.  
//!
//! Supports both Linux and Windows, but help is needed to test other platforms
//!
//! Tested on Rust Stable (1.4), Beta (1.5), and Nightly (1.6)
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

#[derive(Debug)]
pub struct Width(pub u16);
#[derive(Debug)]
pub struct Height(pub u16);

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use crate::unix::{terminal_size, terminal_size_using_fd};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::terminal_size;
