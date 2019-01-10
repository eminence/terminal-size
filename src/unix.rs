extern crate libc;
use std::os::raw::*;
use std::os::unix::io::RawFd;

use super::{Height, Width};

#[cfg(target_os = "macos")]
const TIOCGWINSZ: c_ulong = 0x40087468;
#[cfg(all(target_env = "musl", not(target_os = "macos")))]
const TIOCGWINSZ: c_int = 0x00005413;
#[cfg(all(not(target_env = "musl"), not(target_os = "macos")))]
const TIOCGWINSZ: c_ulong = 0x00005413;

#[derive(Debug)]
struct WinSize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}

/// Returns the size of the terminal defaulting to STDOUT, if available.
///
/// If STDOUT is not a tty, returns `None`
pub fn terminal_size() -> Option<(Width, Height)> {
    use self::libc::STDOUT_FILENO;
    terminal_size_using_fd(STDOUT_FILENO)
}

/// Returns the size of the terminal using the given file descriptor, if available.
///
/// If the given file descriptor is not a tty, returns `None`
pub fn terminal_size_using_fd(fd: RawFd) -> Option<(Width, Height)> {
    use self::libc::ioctl;
    use self::libc::isatty;
    let is_tty: bool = unsafe { isatty(fd) == 1 };

    if !is_tty {
        return None;
    }

    let (rows, cols) = unsafe {
        let mut winsize = WinSize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        ioctl(fd, TIOCGWINSZ, &mut winsize);
        let rows = if winsize.ws_row > 0 {
            winsize.ws_row
        } else {
            0
        };
        let cols = if winsize.ws_col > 0 {
            winsize.ws_col
        } else {
            0
        };
        (rows as u16, cols as u16)
    };

    if rows > 0 && cols > 0 {
        Some((Width(cols), Height(rows)))
    } else {
        None
    }
}

#[test]
/// Compare with the output of `stty size`
fn compare_with_stty() {
    use std::process::Command;
    use std::process::Stdio;

    let output = if cfg!(target_os = "macos") {
        Command::new("stty")
            .arg("-f")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
    } else {
        Command::new("stty")
            .arg("size")
            .arg("-F")
            .arg("/dev/stderr")
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success());
    // stdout is "rows cols"
    let mut data = stdout.split_whitespace();
    let rows = u16::from_str_radix(data.next().unwrap(), 10).unwrap();
    let cols = u16::from_str_radix(data.next().unwrap(), 10).unwrap();
    println!("{}", stdout);
    println!("{} {}", rows, cols);

    if let Some((Width(w), Height(h))) = terminal_size() {
        assert_eq!(rows, h);
        assert_eq!(cols, w);
    }
}
