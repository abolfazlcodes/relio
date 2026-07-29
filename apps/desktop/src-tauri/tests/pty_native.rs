#![cfg(unix)]

use std::ffi::OsString;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use relio_desktop::pty::{LocalPtySession, PortablePtyAdapter, PtySize, ShellProfile};

fn size() -> PtySize {
    PtySize {
        rows: 24,
        columns: 80,
        pixel_width: 640,
        pixel_height: 480,
    }
}

fn shell(arguments: &[&str]) -> ShellProfile {
    ShellProfile {
        id: "native-test".into(),
        display_name: "Native test shell".into(),
        program: PathBuf::from("/bin/sh"),
        arguments: arguments.iter().map(OsString::from).collect(),
        working_directory: None,
    }
}

fn collect_until_exit(session: &LocalPtySession) -> Vec<u8> {
    session
        .grant_output_credit(1024 * 1024)
        .expect("output credit");
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut output = Vec::new();
    while Instant::now() < deadline {
        if let Ok(chunk) = session.receive_output(Duration::from_millis(100)) {
            output.extend_from_slice(&chunk.bytes);
        }
        if session.wait_for_exit(Duration::ZERO).is_some() {
            break;
        }
    }
    output
}

#[test]
fn native_posix_pty_starts_outputs_resizes_and_exits() {
    let profile = shell(&["-c", "printf 'relio-native-pty\\n'"]);
    let session =
        LocalPtySession::start(&PortablePtyAdapter, &profile, size()).expect("native PTY");
    session
        .resize(PtySize {
            rows: 40,
            columns: 120,
            ..size()
        })
        .expect("resize");
    let output = collect_until_exit(&session);
    assert!(
        String::from_utf8_lossy(&output).contains("relio-native-pty"),
        "expected marker in PTY output"
    );
    assert!(
        session
            .wait_for_exit(Duration::from_secs(1))
            .expect("exit result")
            .expect("clean wait")
            .exit_code
            == 0
    );
}

#[test]
fn native_posix_pty_preserves_ordered_input() {
    let profile = shell(&["-c", r#"IFS= read -r line; printf 'received=%s\n' "$line""#]);
    let session =
        LocalPtySession::start(&PortablePtyAdapter, &profile, size()).expect("native PTY");
    session
        .send_input(0, b"ordered-input\n".to_vec())
        .expect("ordered input");
    let output = collect_until_exit(&session);
    assert!(
        String::from_utf8_lossy(&output).contains("received=ordered-input"),
        "shell did not receive the ordered frame"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn forced_stop_does_not_touch_an_unowned_sibling() {
    let mut sibling = std::process::Command::new("/bin/sleep")
        .arg("30")
        .spawn()
        .expect("owned test sibling");
    let profile = ShellProfile {
        id: "sleep-test".into(),
        display_name: "Sleep test".into(),
        program: PathBuf::from("/bin/sleep"),
        arguments: vec![OsString::from("30")],
        working_directory: None,
    };
    let session =
        LocalPtySession::start(&PortablePtyAdapter, &profile, size()).expect("native PTY");
    let _ = session.request_stop();
    assert!(sibling.try_wait().expect("sibling state").is_none());
    sibling.kill().expect("clean up sibling");
    sibling.wait().expect("reap sibling");
}

#[cfg(target_os = "linux")]
#[test]
fn forced_stop_terminates_the_owned_process_group() {
    let profile = shell(&[
        "-c",
        "sleep 30 & child=$!; printf 'child=%s\\n' \"$child\"; wait",
    ]);
    let session =
        LocalPtySession::start(&PortablePtyAdapter, &profile, size()).expect("native PTY");
    session.grant_output_credit(4096).expect("credit");
    let output = session
        .receive_output(Duration::from_secs(2))
        .expect("child PID output")
        .bytes;
    let text = String::from_utf8_lossy(&output);
    let child_pid = text
        .split("child=")
        .nth(1)
        .and_then(|value| {
            value
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
                .ok()
        })
        .expect("child PID");
    let _ = session.request_stop();
    let deadline = Instant::now() + Duration::from_secs(2);
    while PathBuf::from(format!("/proc/{child_pid}")).exists() && Instant::now() < deadline {
        std::thread::yield_now();
    }
    assert!(
        !PathBuf::from(format!("/proc/{child_pid}")).exists(),
        "owned child process remained after forced stop"
    );
}
