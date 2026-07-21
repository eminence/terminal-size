use super::{Height, Width};
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, RawHandle};

/// Returns the size of the terminal.
///
/// This function checks the stdout, stderr, and stdin streams (in that order).
/// The size of the first stream that is attached to a console will be returned.
/// If nothing is attached to a console, then `None` is returned.
///
/// Note that this returns the size of the actual command window, and
/// not the overall size of the command window buffer
pub fn terminal_size() -> Option<(Width, Height)> {
    use windows_sys::Win32::System::Console::{GetStdHandle, STD_HANDLE};

    const HANDLES: [STD_HANDLE; 3] = [
        windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE,
        windows_sys::Win32::System::Console::STD_ERROR_HANDLE,
        windows_sys::Win32::System::Console::STD_INPUT_HANDLE,
    ];

    for handle in HANDLES {
        if let Some(size) = terminal_size_of(unsafe {
            BorrowedHandle::borrow_raw(GetStdHandle(handle) as RawHandle)
        }) {
            return Some(size);
        }
    }
    None
}

/// Returns the size of the terminal using the given handle, if available.
///
/// If the given handle is not attached to a console, returns `None`
pub fn terminal_size_of<Handle: AsHandle>(handle: Handle) -> Option<(Width, Height)> {
    use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
    use windows_sys::Win32::System::Console::{
        GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO,
    };

    // convert between windows_sys::Win32::Foundation::HANDLE and std::os::windows::raw::HANDLE
    let hand = handle.as_handle().as_raw_handle() as windows_sys::Win32::Foundation::HANDLE;

    if hand == INVALID_HANDLE_VALUE {
        return None;
    }

    let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = unsafe { std::mem::zeroed() };

    if unsafe { GetConsoleScreenBufferInfo(hand, &mut csbi) } == 0 {
        return None;
    }

    Some((
        Width((csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16),
        Height((csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16),
    ))
}

/// Returns the size of the terminal using the given handle, if available.
///
/// The given handle must be an open handle.
///
/// If the given handle is not a tty, returns `None`
///
/// # Safety
///
/// `handle` must be a valid open file handle.
#[deprecated(note = "Use `terminal_size_of` instead.
     Use `BorrowedHandle::borrow_raw` to convert a raw handle into a `BorrowedHandle` if needed.")]
pub unsafe fn terminal_size_using_handle(handle: RawHandle) -> Option<(Width, Height)> {
    terminal_size_of(BorrowedHandle::borrow_raw(handle))
}
