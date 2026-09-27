#![cfg(unix)]

use std::fs::File;
use std::process::{Command, Stdio};
use terminal_size::{terminal_size, terminal_size_of};

#[test]
fn redirected_streams_use_controlling_terminal() {
    const CHILD: &str = "TERMINAL_SIZE_TEST_REDIRECTED_CHILD";
    const REQUIRE_TTY: &str = "TERMINAL_SIZE_TEST_REQUIRE_TTY";
    const SUCCESS_MARKER: &str = "controlling terminal fallback checked";

    if std::env::var_os(CHILD).is_some() {
        let tty = File::open("/dev/tty").expect("child should have a controlling terminal");
        let expected = terminal_size_of(&tty).expect("controlling terminal should have a size");

        assert_eq!(terminal_size_of(std::io::stdout()), None);
        assert_eq!(terminal_size_of(std::io::stderr()), None);
        assert_eq!(terminal_size_of(std::io::stdin()), None);
        assert_eq!(terminal_size(), Some(expected));
        println!("{SUCCESS_MARKER}");
        return;
    }

    let tty = match File::open("/dev/tty") {
        Ok(tty) => tty,
        Err(error) => {
            assert!(
                std::env::var_os(REQUIRE_TTY).is_none(),
                "controlling terminal required: {error}"
            );
            eprintln!("skipping: no controlling terminal");
            return;
        }
    };
    if terminal_size_of(&tty).is_none() {
        assert!(
            std::env::var_os(REQUIRE_TTY).is_none(),
            "controlling terminal must have a nonzero size"
        );
        eprintln!("skipping: controlling terminal has no size");
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("redirected_streams_use_controlling_terminal")
        .arg("--nocapture")
        .env(CHILD, "1")
        .stdin(Stdio::null())
        .output()
        .unwrap();

    assert!(
        output.status.success() && String::from_utf8_lossy(&output.stdout).contains(SUCCESS_MARKER),
        "child failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
