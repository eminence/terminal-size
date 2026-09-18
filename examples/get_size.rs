#[cfg(any(
    all(
        unix,
        not(any(
            target_os = "espidf",
            target_os = "horizon",
            target_os = "vita",
            target_os = "wasi",
        ))
    ),
    windows,
    target_os = "hermit",
    target_os = "motor",
    target_os = "trusty",
    target_os = "wasi"
))]
fn main() {
    println!(
        "Size from terminal_size():                    {:?}",
        terminal_size::terminal_size()
    );

    println!(
        "Size from terminal_size_of(stdout):           {:?}",
        terminal_size::terminal_size_of(std::io::stdout())
    );
    println!(
        "Size from terminal_size_of(stderr):           {:?}",
        terminal_size::terminal_size_of(std::io::stderr())
    );
    println!(
        "Size from terminal_size_of(stdin):            {:?}",
        terminal_size::terminal_size_of(std::io::stdin())
    );
}

#[cfg(not(any(
    all(
        unix,
        not(any(
            target_os = "espidf",
            target_os = "horizon",
            target_os = "vita",
            target_os = "wasi",
        ))
    ),
    windows,
    target_os = "hermit",
    target_os = "motor",
    target_os = "trusty",
    target_os = "wasi"
)))]
fn main() {}
