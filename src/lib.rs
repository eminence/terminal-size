
//! A simple utility for getting the size of a terminal.
//!
//! Works on Linux, macOS, Windows, and illumos.
//!
//! This crate requires a minimum Rust version of 1.63.0 (2022-08-11).
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
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Width(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Height(pub u16);

impl Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {}", self.0)
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Height: {}", self.0)
    }
}

impl From<Width> for u16 {
    fn from(width: Width) -> Self {
        width.0
    }
}

impl From<Height> for u16 {
    fn from(height: Height) -> Self {
        height.0
    }
}

impl Add for Width {
    type Output = Width;

    fn add(self, other: Width) -> Width {
        Width(self.0 + other.0)
    }
}

impl Sub for Width {
    type Output = Width;

    fn sub(self, other: Width) -> Width {
        Width(self.0 - other.0)
    }
}

impl Mul<u16> for Width {
    type Output = Width;

    fn mul(self, rhs: u16) -> Width {
        Width(self.0 * rhs)
    }
}

impl Div<u16> for Width {
    type Output = Width;

    fn div(self, rhs: u16) -> Width {
        Width(self.0 / rhs)
    }
}

impl Add for Height {
    type Output = Height;

    fn add(self, other: Height) -> Height {
        Height(self.0 + other.0)
    }
}

impl Sub for Height {
    type Output = Height;

    fn sub(self, other: Height) -> Height {
        Height(self.0 - other.0)
    }
}

impl Mul<u16> for Height {
    type Output = Height;

    fn mul(self, rhs: u16) -> Height {
        Height(self.0 * rhs)
    }
}

impl Div<u16> for Height {
    type Output = Height;

    fn div(self, rhs: u16) -> Height {
        Height(self.0 / rhs)
    }
}

#[cfg(unix)]
mod unix;

#[cfg(unix)]
#[allow(deprecated)]
pub use crate::unix::{terminal_size, terminal_size_of, terminal_size_using_fd};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
#[allow(deprecated)]
pub use crate::windows::{terminal_size, terminal_size_of, terminal_size_using_handle};

#[cfg(not(any(unix, windows)))]
pub fn terminal_size() -> Option<(Width, Height)> {
    None
}
