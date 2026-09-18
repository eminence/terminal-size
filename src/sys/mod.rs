#[cfg(any(
    unix,
    target_os = "hermit",
    target_os = "motor",
    target_os = "trusty",
    target_os = "wasi"
))]
pub mod fd;

#[cfg(windows)]
pub mod windows;

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
pub mod unsupported;
