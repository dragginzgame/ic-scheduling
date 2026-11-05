//! Explicit process-level qualification of the real terminal and owned server.
#![cfg(unix)]

use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use ic_host_process::testing::write_executable_script;

#[derive(Clone, Copy, Debug)]
enum Shutdown {
    Quit,
    Eof,
    InvalidCommand,
    Interrupt,
    WatchInterrupt,
}

struct Session {
    child: Child,
    server_pid: PathBuf,
}

fn signal(pid: &str, signal: &str) -> bool {
    Command::new("/bin/kill")
        .args([signal, pid])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("invoke kill")
        .success()
}

impl Drop for Session {
    fn drop(&mut self) {
        // Failure containment only: assertions below observe natural cleanup first.
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Ok(pid) = fs::read_to_string(&self.server_pid) {
            signal(pid.trim(), "-KILL");
        }
    }
}

fn until(mut condition: impl FnMut() -> bool, description: &str) {
    let deadline = Instant::now() + Duration::from_secs(45);
    while !condition() {
        assert!(Instant::now() < deadline, "timed out: {description}");
        thread::sleep(Duration::from_millis(50));
    }
}

#[test]
#[ignore = "explicit real CLI/server shutdown: make test-demo-cli"]
fn quit_eof_errors_and_interrupts_release_owned_server() {
    let binary = std::env::var_os("IC_JOBS_QUEUE_BIN").expect("queue binary");
    let server = std::env::var_os("IC_JOBS_QUEUE_SERVER").expect("admitted server binary");
    let wasm = std::env::var_os("IC_JOBS_CONSUMER_WASM").expect("consumer Wasm");
    let configured = std::env::var_os("TMPDIR");
    let parent = configured.as_ref().map_or_else(
        || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/tmp"),
        PathBuf::from,
    );
    assert!(
        !parent
            .as_os_str()
            .as_encoded_bytes()
            .iter()
            .any(|byte| matches!(byte, b'\r' | b'\n')),
        "scratch directory contains CR/LF"
    );
    if configured.is_none() {
        fs::create_dir_all(&parent).unwrap();
    }
    for mode in [
        Shutdown::Quit,
        Shutdown::Eof,
        Shutdown::InvalidCommand,
        Shutdown::Interrupt,
        Shutdown::WatchInterrupt,
    ] {
        let fixture =
            ic_host_fs::testing::FixtureDirectory::new(&parent, "jobs-queue-cli").unwrap();
        let root = fixture.path();
        let wrapper = root.join("server");
        // exec preserves the PID while leaving all server arguments untouched.
        write_executable_script(&wrapper, "#!/bin/sh\nprintf '%s\\n' \"$$\" > \"$JOBS_SERVER_PID\"\nexec \"$JOBS_REAL_SERVER\" \"$@\"\n").unwrap();
        let log_path = root.join("terminal.log");
        let log = fs::File::create(&log_path).unwrap();
        let server_pid = root.join("server.pid");
        let child = Command::new(&binary)
            .arg("--server")
            .arg(&wrapper)
            .arg("--wasm")
            .arg(&wasm)
            .env("JOBS_REAL_SERVER", &server)
            .env("JOBS_SERVER_PID", &server_pid)
            .stdin(Stdio::piped())
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .spawn()
            .unwrap();
        let mut session = Session { child, server_pid };
        let mut stdin = session.child.stdin.take().unwrap();
        writeln!(stdin, "list").unwrap();
        until(
            || {
                let output = fs::read_to_string(&log_path).unwrap();
                assert!(
                    session.child.try_wait().unwrap().is_none(),
                    "{mode:?}: {output}"
                );
                output.contains("Next wakeup:")
            },
            "canister installed and terminal accepting commands",
        );
        let pid = fs::read_to_string(&session.server_pid).unwrap();
        assert!(signal(pid.trim(), "-0"), "server must be running");
        match mode {
            Shutdown::Quit => writeln!(stdin, "quit").unwrap(),
            Shutdown::Eof => drop(stdin),
            Shutdown::InvalidCommand => writeln!(stdin, "unknown-command").unwrap(),
            Shutdown::Interrupt => assert!(signal(&session.child.id().to_string(), "-INT")),
            Shutdown::WatchInterrupt => {
                writeln!(stdin, "add after-stop --after 1s\nwatch 1w").unwrap();
                until(
                    || fs::read_to_string(&log_path).unwrap().contains("Watching;"),
                    "watch entered before its first advance",
                );
                assert!(signal(&session.child.id().to_string(), "-INT"));
            }
        }
        until(
            || session.child.try_wait().unwrap().is_some(),
            "terminal exit",
        );
        assert!(
            session.child.wait().unwrap().success() == !matches!(mode, Shutdown::InvalidCommand),
            "{mode:?}: {}",
            fs::read_to_string(&log_path).unwrap()
        );
        until(|| !signal(pid.trim(), "-0"), "owned server exit");
        if matches!(mode, Shutdown::WatchInterrupt) {
            assert!(
                !fs::read_to_string(&log_path).unwrap().contains("RAN #"),
                "watch dispatched work after Ctrl+C"
            );
        }
        // Do not send cleanup signals after successful natural termination.
        fs::remove_file(&session.server_pid).unwrap();
    }
}
