use crate::WindowSize;

use super::{Height, Width};
use rustix::fd::{AsFd, BorrowedFd};
use std::os::unix::io::RawFd;

/// Returns the size of the terminal.
///
/// This function checks the stdout, stderr, and stdin streams (in that order).
/// The size of the first stream that is a TTY will be returned.  If nothing
/// is a TTY, then `None` is returned.
pub fn terminal_size() -> Option<(Width, Height)> {
    terminal_size_full().map(|(width, height, _)| (width, height))
}

/// Returns the full size of the terminal.
///
/// This function checks the stdout, stderr, and stdin streams (in that order).
/// The size of the first stream that is a TTY will be returned.  If nothing
/// is a TTY, then `None` is returned.
pub fn terminal_size_full() -> Option<(Width, Height, Option<WindowSize>)> {
    terminal_size_full_from_fd(std::io::stdout())
        .or_else(|| terminal_size_full_from_fd(std::io::stderr()))
        .or_else(|| terminal_size_full_from_fd(std::io::stdin()))
}

/// Returns the full size of the terminal under the given file descriptor, if available.
///
/// If the file descriptor is not a TTY return `None`; if the underlying terminal does not
/// report a reasonable window size, the 3rd item in the return value is `None`.
pub fn terminal_size_full_from_fd<F: AsFd>(fd: F) -> Option<(Width, Height, Option<WindowSize>)> {
    use rustix::termios::{isatty, tcgetwinsize};

    if !isatty(fd.as_fd()) {
        return None;
    }

    let winsize = tcgetwinsize(fd.as_fd()).ok()?;

    let rows = winsize.ws_row;
    let cols = winsize.ws_col;
    let window_size = if winsize.ws_xpixel != 0 && winsize.ws_ypixel != 0 {
        Some(WindowSize {
            x: winsize.ws_xpixel,
            y: winsize.ws_ypixel,
        })
    } else {
        None
    };

    if rows > 0 && cols > 0 {
        Some((Width(cols), Height(rows), window_size))
    } else {
        None
    }
}

/// Returns the size of the terminal using the given file descriptor, if available.
///
/// This function is not IO-safe because it takes a raw file descriptor.  Use
/// [`terminal_size_full_from_fd`] instead.
///
/// If the given file descriptor is not a tty, returns `None`.
#[deprecated(
    since = "0.4.0",
    note = "Not IO-safe, use terminal_size_full_from_fd instead"
)]
pub fn terminal_size_using_fd(fd: RawFd) -> Option<(Width, Height)> {
    // TODO: Once I/O safety is stabilized, the enlosing function here should
    // be unsafe due to taking a `RawFd`. We should then move the main
    // logic here into a new function which takes a `BorrowedFd` and is safe.
    let fd = unsafe { BorrowedFd::borrow_raw(fd) };
    terminal_size_full_from_fd(fd).map(|(width, height, _)| (width, height))
}

#[test]
/// Compare with the output of `stty size`
fn compare_with_stty() {
    use std::process::Command;
    use std::process::Stdio;

    let (rows, cols) = if cfg!(target_os = "illumos") {
        // illumos stty(1) does not accept a device argument, instead using
        // stdin unconditionally:
        let output = Command::new("stty")
            .stdin(Stdio::inherit())
            .output()
            .unwrap();
        assert!(output.status.success());

        // stdout includes the row and columns thus: "rows = 80; columns = 24;"
        let vals = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .flat_map(|line| {
                // Split each line on semicolons to get "k = v" strings:
                line.split(';')
                    .map(str::trim)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .filter_map(|term| {
                // split each "k = v" string and look for rows/columns:
                match term.splitn(2, " = ").collect::<Vec<_>>().as_slice() {
                    ["rows", n] | ["columns", n] => Some(n.parse().unwrap()),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        (vals[0], vals[1])
    } else {
        let output = if cfg!(target_os = "linux") {
            Command::new("stty")
                .arg("size")
                .arg("-F")
                .arg("/dev/stderr")
                .stderr(Stdio::inherit())
                .output()
                .unwrap()
        } else {
            Command::new("stty")
                .arg("-f")
                .arg("/dev/stderr")
                .arg("size")
                .stderr(Stdio::inherit())
                .output()
                .unwrap()
        };

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        // stdout is "rows cols"
        let mut data = stdout.split_whitespace();
        println!("{}", stdout);
        let rows = data.next().unwrap().parse::<u16>().unwrap();
        let cols = data.next().unwrap().parse::<u16>().unwrap();
        (rows, cols)
    };
    println!("{} {}", rows, cols);

    if let Some((Width(w), Height(h))) = terminal_size() {
        assert_eq!(rows, h);
        assert_eq!(cols, w);
    } else {
        panic!("terminal_size() return None");
    }
}
