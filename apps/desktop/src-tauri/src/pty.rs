use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender, TrySendError};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use portable_pty::{CommandBuilder, PtySize as NativePtySize};
use uuid::Uuid;

pub const MAX_OUTPUT_CHUNK_BYTES: usize = 64 * 1024;
pub const MAX_OUTPUT_CREDIT_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_INPUT_FRAME_BYTES: usize = 64 * 1024;
pub const MAX_PENDING_INPUT_BYTES: usize = 1024 * 1024;
pub const OUTPUT_CHANNEL_CHUNKS: usize = 16;
pub const INPUT_CHANNEL_FRAMES: usize = 64;
pub const LOCAL_SESSION_STOP_DEADLINE: Duration = Duration::from_secs(3);
const MAX_PROFILE_ARGUMENTS: usize = 16;
const MAX_PROFILE_VALUE_BYTES: usize = 4 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PtySize {
    pub rows: u16,
    pub columns: u16,
    pub pixel_width: u16,
    pub pixel_height: u16,
}

impl PtySize {
    pub fn validate(self) -> Result<Self, PtyError> {
        if self.rows == 0 || self.columns == 0 || self.rows > 1_000 || self.columns > 1_000 {
            return Err(PtyError::InvalidDimensions);
        }
        Ok(self)
    }
}

impl From<PtySize> for NativePtySize {
    fn from(value: PtySize) -> Self {
        Self {
            rows: value.rows,
            cols: value.columns,
            pixel_width: value.pixel_width,
            pixel_height: value.pixel_height,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShellProfile {
    pub id: String,
    pub display_name: String,
    pub program: PathBuf,
    pub arguments: Vec<OsString>,
    pub working_directory: Option<PathBuf>,
}

impl ShellProfile {
    pub fn validate(self) -> Result<Self, PtyError> {
        if self.id.is_empty()
            || self.id.len() > 64
            || !self.id.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_' | b'-')
            })
            || self.display_name.is_empty()
            || self.display_name.len() > 128
            || !self.program.is_absolute()
            || !is_executable_file(&self.program)
            || self.arguments.len() > MAX_PROFILE_ARGUMENTS
            || self
                .arguments
                .iter()
                .any(|argument| !valid_os_value(argument))
            || self
                .working_directory
                .as_ref()
                .is_some_and(|path| !path.is_absolute() || !path.is_dir())
        {
            return Err(PtyError::InvalidShellProfile);
        }
        Ok(self)
    }
}

#[cfg(unix)]
fn valid_os_value(value: &std::ffi::OsStr) -> bool {
    use std::os::unix::ffi::OsStrExt;
    let bytes = value.as_bytes();
    bytes.len() <= MAX_PROFILE_VALUE_BYTES && !bytes.contains(&0)
}

#[cfg(windows)]
fn valid_os_value(value: &std::ffi::OsStr) -> bool {
    use std::os::windows::ffi::OsStrExt;
    let mut length = 0_usize;
    for unit in value.encode_wide() {
        if unit == 0 {
            return false;
        }
        length = length.saturating_add(2);
    }
    length <= MAX_PROFILE_VALUE_BYTES
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(windows)]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
}

#[must_use]
pub fn discover_shell_profiles() -> Vec<ShellProfile> {
    let mut candidates = Vec::new();
    #[cfg(unix)]
    {
        if let Some(shell) = std::env::var_os("SHELL").map(PathBuf::from) {
            candidates.push(("default", "Default shell", shell, Vec::new()));
        }
        candidates.extend([
            ("zsh", "Z shell", PathBuf::from("/bin/zsh"), Vec::new()),
            ("bash", "Bash", PathBuf::from("/bin/bash"), Vec::new()),
            ("sh", "POSIX shell", PathBuf::from("/bin/sh"), Vec::new()),
        ]);
    }
    #[cfg(windows)]
    {
        if let Some(program_files) = std::env::var_os("ProgramFiles").map(PathBuf::from) {
            candidates.push((
                "pwsh",
                "PowerShell",
                program_files.join("PowerShell/7/pwsh.exe"),
                vec![OsString::from("-NoLogo")],
            ));
        }
        if let Some(system_root) = std::env::var_os("SystemRoot").map(PathBuf::from) {
            candidates.push((
                "windows-powershell",
                "Windows PowerShell",
                system_root.join("System32/WindowsPowerShell/v1.0/powershell.exe"),
                vec![OsString::from("-NoLogo")],
            ));
            candidates.push((
                "cmd",
                "Command Prompt",
                system_root.join("System32/cmd.exe"),
                Vec::new(),
            ));
        }
    }
    let mut seen = BTreeMap::<PathBuf, ()>::new();
    candidates
        .into_iter()
        .filter_map(|(id, display_name, program, arguments)| {
            if seen.insert(program.clone(), ()).is_some() {
                return None;
            }
            ShellProfile {
                id: id.into(),
                display_name: display_name.into(),
                program,
                arguments,
                working_directory: None,
            }
            .validate()
            .ok()
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputChunk {
    pub sequence: u64,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PtyExit {
    pub exit_code: u32,
    pub signal: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PtySessionState {
    Starting,
    Running,
    Stopping,
    Exited,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PtyError {
    #[error("PTY dimensions are invalid")]
    InvalidDimensions,
    #[error("shell profile is invalid or unavailable")]
    InvalidShellProfile,
    #[error("PTY is unavailable on this platform")]
    UnsupportedPlatform,
    #[error("PTY could not be created")]
    OpenFailed,
    #[error("shell could not be started")]
    SpawnFailed,
    #[error("PTY stream could not be created")]
    StreamFailed,
    #[error("terminal output credit is invalid")]
    InvalidCredit,
    #[error("terminal input frame is invalid")]
    InvalidInput,
    #[error("terminal input sequence is out of order")]
    InputSequence,
    #[error("terminal input queue is full")]
    InputBackpressure,
    #[error("terminal output is unavailable")]
    OutputClosed,
    #[error("terminal output is not ready")]
    OutputPending,
    #[error("PTY resize failed")]
    ResizeFailed,
    #[error("shell termination failed")]
    TerminationFailed,
    #[error("PTY session is no longer active")]
    SessionClosed,
}

pub trait ResizeHandle: Send {
    fn resize(&mut self, size: PtySize) -> Result<(), PtyError>;
}

pub trait KillHandle: Send {
    fn kill(&mut self) -> Result<(), PtyError>;
}

pub trait WaitHandle: Send {
    fn wait(&mut self) -> Result<PtyExit, PtyError>;
}

pub struct SpawnedPty {
    pub reader: Box<dyn Read + Send>,
    pub writer: Box<dyn Write + Send>,
    pub resize: Box<dyn ResizeHandle>,
    pub killer: Box<dyn KillHandle>,
    pub waiter: Box<dyn WaitHandle>,
    pub process_id: Option<u32>,
}

pub trait PtyAdapter: Send + Sync {
    fn spawn(
        &self,
        session_id: Uuid,
        profile: &ShellProfile,
        size: PtySize,
    ) -> Result<SpawnedPty, PtyError>;
}

#[derive(Default)]
pub struct PortablePtyAdapter;

impl PtyAdapter for PortablePtyAdapter {
    fn spawn(
        &self,
        session_id: Uuid,
        profile: &ShellProfile,
        size: PtySize,
    ) -> Result<SpawnedPty, PtyError> {
        let profile = profile.clone().validate()?;
        let size = size.validate()?;
        let pair = portable_pty::native_pty_system()
            .openpty(size.into())
            .map_err(|_| PtyError::OpenFailed)?;
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|_| PtyError::StreamFailed)?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|_| PtyError::StreamFailed)?;
        let mut command = CommandBuilder::new(&profile.program);
        command.args(&profile.arguments);
        command.env_clear();
        for (key, value) in sanitized_environment() {
            command.env(key, value);
        }
        command.env("TERM", "xterm-256color");
        command.env("COLORTERM", "truecolor");
        command.env("RELIO_SESSION_ID", session_id.to_string());
        if let Some(directory) = profile.working_directory {
            command.cwd(directory);
        }
        let child = pair
            .slave
            .spawn_command(command)
            .map_err(|_| PtyError::SpawnFailed)?;
        drop(pair.slave);
        let process_id = child.process_id();
        #[cfg(unix)]
        let process_group = pair.master.process_group_leader();
        #[cfg(windows)]
        let process_job = create_process_job(&*child)?;
        let killer = child.clone_killer();
        Ok(SpawnedPty {
            reader,
            writer,
            resize: Box::new(PortableResize {
                master: pair.master,
            }),
            killer: Box::new(PortableKiller {
                inner: killer,
                #[cfg(unix)]
                process_group,
                #[cfg(windows)]
                process_job: Some(process_job),
            }),
            waiter: Box::new(PortableWaiter { inner: child }),
            process_id,
        })
    }
}

fn sanitized_environment() -> Vec<(OsString, OsString)> {
    std::env::vars_os()
        .filter(|(key, value)| {
            let key = key.to_string_lossy();
            (matches!(
                key.as_ref(),
                "HOME"
                    | "USER"
                    | "LOGNAME"
                    | "PATH"
                    | "SHELL"
                    | "LANG"
                    | "TMPDIR"
                    | "XDG_CONFIG_HOME"
                    | "XDG_DATA_HOME"
                    | "XDG_CACHE_HOME"
                    | "SystemRoot"
                    | "USERPROFILE"
                    | "APPDATA"
                    | "LOCALAPPDATA"
                    | "TEMP"
                    | "TMP"
                    | "ComSpec"
                    | "PATHEXT"
            ) || key.starts_with("LC_"))
                && valid_os_value(value)
        })
        .collect()
}

struct PortableResize {
    master: Box<dyn portable_pty::MasterPty + Send>,
}

impl ResizeHandle for PortableResize {
    fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
        self.master
            .resize(size.validate()?.into())
            .map_err(|_| PtyError::ResizeFailed)
    }
}

struct PortableKiller {
    inner: Box<dyn portable_pty::ChildKiller + Send + Sync>,
    #[cfg(unix)]
    process_group: Option<i32>,
    #[cfg(windows)]
    process_job: Option<win32job::Job>,
}

impl KillHandle for PortableKiller {
    fn kill(&mut self) -> Result<(), PtyError> {
        #[cfg(unix)]
        if let Some(group) = self.process_group {
            use nix::sys::signal::{Signal, killpg};
            use nix::unistd::Pid;
            return killpg(Pid::from_raw(group), Signal::SIGKILL)
                .map_err(|_| PtyError::TerminationFailed);
        }
        #[cfg(windows)]
        if self.process_job.take().is_some() {
            return Ok(());
        }
        self.inner.kill().map_err(|_| PtyError::TerminationFailed)
    }
}

#[cfg(windows)]
fn create_process_job(
    child: &(dyn portable_pty::Child + Send + Sync),
) -> Result<win32job::Job, PtyError> {
    let handle = child.as_raw_handle().ok_or(PtyError::SpawnFailed)?;
    let mut limits = win32job::ExtendedLimitInfo::new();
    limits.limit_kill_on_job_close();
    let job = win32job::Job::create_with_limit_info(&limits).map_err(|_| PtyError::SpawnFailed)?;
    job.assign_process(handle as isize)
        .map_err(|_| PtyError::SpawnFailed)?;
    Ok(job)
}

struct PortableWaiter {
    inner: Box<dyn portable_pty::Child + Send + Sync>,
}

impl WaitHandle for PortableWaiter {
    fn wait(&mut self) -> Result<PtyExit, PtyError> {
        self.inner
            .wait()
            .map(|status| PtyExit {
                exit_code: status.exit_code(),
                signal: status.signal().map(str::to_owned),
            })
            .map_err(|_| PtyError::SessionClosed)
    }
}

struct OutputFlow {
    state: Mutex<OutputFlowState>,
    changed: Condvar,
}

#[derive(Default)]
struct OutputFlowState {
    credit: usize,
    stopped: bool,
}

impl OutputFlow {
    fn grant(&self, bytes: usize) -> Result<(), PtyError> {
        if bytes == 0 || bytes > MAX_OUTPUT_CREDIT_BYTES {
            return Err(PtyError::InvalidCredit);
        }
        let mut state = self.state.lock().map_err(|_| PtyError::SessionClosed)?;
        state.credit = state
            .credit
            .checked_add(bytes)
            .filter(|credit| *credit <= MAX_OUTPUT_CREDIT_BYTES)
            .ok_or(PtyError::InvalidCredit)?;
        self.changed.notify_one();
        Ok(())
    }

    fn reserve(&self) -> Option<usize> {
        let mut state = self.state.lock().ok()?;
        while state.credit == 0 && !state.stopped {
            state = self.changed.wait(state).ok()?;
        }
        if state.stopped {
            return None;
        }
        let reserved = state.credit.min(MAX_OUTPUT_CHUNK_BYTES);
        state.credit -= reserved;
        Some(reserved)
    }

    fn refund(&self, bytes: usize) {
        if let Ok(mut state) = self.state.lock() {
            state.credit = state
                .credit
                .saturating_add(bytes)
                .min(MAX_OUTPUT_CREDIT_BYTES);
        }
    }

    fn stop(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.stopped = true;
            self.changed.notify_all();
        }
    }
}

enum InputMessage {
    Data { bytes: Vec<u8> },
    Close,
}

#[derive(Default)]
struct ExitState {
    result: Option<Result<PtyExit, PtyError>>,
}

pub struct LocalPtySession {
    id: Uuid,
    input: SyncSender<InputMessage>,
    output: Mutex<Receiver<OutputChunk>>,
    flow: Arc<OutputFlow>,
    pending_input: Arc<AtomicUsize>,
    next_input_sequence: Mutex<u64>,
    resize: Mutex<Box<dyn ResizeHandle>>,
    killer: Arc<Mutex<Box<dyn KillHandle>>>,
    exit: Arc<(Mutex<ExitState>, Condvar)>,
    state: Arc<Mutex<PtySessionState>>,
    stop_requested: AtomicBool,
    process_id: Option<u32>,
}

impl LocalPtySession {
    pub fn start(
        adapter: &dyn PtyAdapter,
        profile: &ShellProfile,
        size: PtySize,
    ) -> Result<Self, PtyError> {
        let id = Uuid::now_v7();
        let spawned = adapter.spawn(id, profile, size)?;
        let (input_tx, input_rx) = mpsc::sync_channel(INPUT_CHANNEL_FRAMES);
        let (output_tx, output_rx) = mpsc::sync_channel(OUTPUT_CHANNEL_CHUNKS);
        let pending_input = Arc::new(AtomicUsize::new(0));
        let flow = Arc::new(OutputFlow {
            state: Mutex::new(OutputFlowState::default()),
            changed: Condvar::new(),
        });
        let exit = Arc::new((Mutex::new(ExitState::default()), Condvar::new()));
        let state = Arc::new(Mutex::new(PtySessionState::Running));
        spawn_input_pump(spawned.writer, input_rx, Arc::clone(&pending_input));
        spawn_output_pump(spawned.reader, output_tx, Arc::clone(&flow));
        spawn_waiter(spawned.waiter, Arc::clone(&exit), Arc::clone(&state));
        Ok(Self {
            id,
            input: input_tx,
            output: Mutex::new(output_rx),
            flow,
            pending_input,
            next_input_sequence: Mutex::new(0),
            resize: Mutex::new(spawned.resize),
            killer: Arc::new(Mutex::new(spawned.killer)),
            exit,
            state,
            stop_requested: AtomicBool::new(false),
            process_id: spawned.process_id,
        })
    }

    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    #[must_use]
    pub const fn process_id(&self) -> Option<u32> {
        self.process_id
    }

    #[must_use]
    pub fn state(&self) -> PtySessionState {
        self.state
            .lock()
            .map_or(PtySessionState::Failed, |state| *state)
    }

    pub fn grant_output_credit(&self, bytes: usize) -> Result<(), PtyError> {
        self.flow.grant(bytes)
    }

    pub fn receive_output(&self, timeout: Duration) -> Result<OutputChunk, PtyError> {
        self.output
            .lock()
            .map_err(|_| PtyError::OutputClosed)?
            .recv_timeout(timeout)
            .map_err(|error| match error {
                RecvTimeoutError::Timeout => PtyError::OutputPending,
                RecvTimeoutError::Disconnected => PtyError::OutputClosed,
            })
    }

    pub fn send_input(&self, sequence: u64, bytes: Vec<u8>) -> Result<(), PtyError> {
        if self.state() != PtySessionState::Running {
            return Err(PtyError::SessionClosed);
        }
        if bytes.is_empty() || bytes.len() > MAX_INPUT_FRAME_BYTES {
            return Err(PtyError::InvalidInput);
        }
        let mut expected = self
            .next_input_sequence
            .lock()
            .map_err(|_| PtyError::SessionClosed)?;
        if sequence != *expected {
            return Err(PtyError::InputSequence);
        }
        let length = bytes.len();
        self.pending_input
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |pending| {
                pending
                    .checked_add(length)
                    .filter(|next| *next <= MAX_PENDING_INPUT_BYTES)
            })
            .map_err(|_| PtyError::InputBackpressure)?;
        match self.input.try_send(InputMessage::Data { bytes }) {
            Ok(()) => {
                *expected = expected.saturating_add(1);
                Ok(())
            }
            Err(TrySendError::Full(_)) => {
                self.pending_input.fetch_sub(length, Ordering::AcqRel);
                Err(PtyError::InputBackpressure)
            }
            Err(TrySendError::Disconnected(_)) => {
                self.pending_input.fetch_sub(length, Ordering::AcqRel);
                Err(PtyError::SessionClosed)
            }
        }
    }

    pub fn resize(&self, size: PtySize) -> Result<(), PtyError> {
        if self.state() != PtySessionState::Running {
            return Err(PtyError::SessionClosed);
        }
        self.resize
            .lock()
            .map_err(|_| PtyError::SessionClosed)?
            .resize(size)
    }

    pub fn wait_for_exit(&self, timeout: Duration) -> Option<Result<PtyExit, PtyError>> {
        let deadline = Instant::now() + timeout;
        let (lock, changed) = &*self.exit;
        let mut exit = lock.lock().ok()?;
        while exit.result.is_none() {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return None;
            }
            let (next, wait) = changed.wait_timeout(exit, remaining).ok()?;
            exit = next;
            if wait.timed_out() && exit.result.is_none() {
                return None;
            }
        }
        exit.result.clone()
    }

    pub fn request_stop(&self) -> Result<PtyExit, PtyError> {
        if let Some(result) = self.wait_for_exit(Duration::ZERO) {
            return result;
        }
        if self.stop_requested.swap(true, Ordering::AcqRel) {
            return self
                .wait_for_exit(LOCAL_SESSION_STOP_DEADLINE)
                .unwrap_or(Err(PtyError::TerminationFailed));
        }
        if let Ok(mut state) = self.state.lock() {
            *state = PtySessionState::Stopping;
        }
        self.flow.stop();
        let _ = self.input.try_send(InputMessage::Close);
        if let Some(result) = self.wait_for_exit(LOCAL_SESSION_STOP_DEADLINE) {
            return result;
        }
        self.killer
            .lock()
            .map_err(|_| PtyError::TerminationFailed)?
            .kill()?;
        self.wait_for_exit(LOCAL_SESSION_STOP_DEADLINE)
            .unwrap_or(Err(PtyError::TerminationFailed))
    }
}

impl Drop for LocalPtySession {
    fn drop(&mut self) {
        let _ = self.request_stop();
    }
}

fn spawn_input_pump(
    mut writer: Box<dyn Write + Send>,
    receiver: Receiver<InputMessage>,
    pending: Arc<AtomicUsize>,
) {
    let _ = thread::Builder::new()
        .name("relio-pty-input".into())
        .spawn(move || {
            while let Ok(message) = receiver.recv() {
                match message {
                    InputMessage::Data { bytes } => {
                        let length = bytes.len();
                        let result = writer.write_all(&bytes).and_then(|()| writer.flush());
                        pending.fetch_sub(length, Ordering::AcqRel);
                        if result.is_err() {
                            break;
                        }
                    }
                    InputMessage::Close => break,
                }
            }
        });
}

fn spawn_output_pump(
    mut reader: Box<dyn Read + Send>,
    sender: SyncSender<OutputChunk>,
    flow: Arc<OutputFlow>,
) {
    let _ = thread::Builder::new()
        .name("relio-pty-output".into())
        .spawn(move || {
            let mut sequence = 0_u64;
            while let Some(reserved) = flow.reserve() {
                let mut bytes = vec![0_u8; reserved];
                match reader.read(&mut bytes) {
                    Ok(0) | Err(_) => break,
                    Ok(read) => {
                        bytes.truncate(read);
                        flow.refund(reserved - read);
                        if sender.send(OutputChunk { sequence, bytes }).is_err() {
                            break;
                        }
                        sequence = sequence.saturating_add(1);
                    }
                }
            }
        });
}

fn spawn_waiter(
    mut waiter: Box<dyn WaitHandle>,
    exit: Arc<(Mutex<ExitState>, Condvar)>,
    state: Arc<Mutex<PtySessionState>>,
) {
    let _ = thread::Builder::new()
        .name("relio-pty-wait".into())
        .spawn(move || {
            let result = waiter.wait();
            if let Ok(mut session_state) = state.lock() {
                *session_state = if result.is_ok() {
                    PtySessionState::Exited
                } else {
                    PtySessionState::Failed
                };
            }
            let (lock, changed) = &*exit;
            if let Ok(mut exit_state) = lock.lock() {
                exit_state.result = Some(result);
                changed.notify_all();
            }
        });
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::sync::atomic::{AtomicU16, AtomicU32};

    use super::*;

    #[derive(Default)]
    struct FakeAdapter {
        writes: Arc<Mutex<Vec<u8>>>,
        resized_rows: Arc<AtomicU16>,
        kills: Arc<AtomicU32>,
    }

    struct FakeWriter(Arc<Mutex<Vec<u8>>>);

    impl Write for FakeWriter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.lock().expect("writes").extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct FakeResize(Arc<AtomicU16>);

    impl ResizeHandle for FakeResize {
        fn resize(&mut self, size: PtySize) -> Result<(), PtyError> {
            self.0.store(size.rows, Ordering::Release);
            Ok(())
        }
    }

    struct FakeKiller(Arc<AtomicU32>);

    impl KillHandle for FakeKiller {
        fn kill(&mut self) -> Result<(), PtyError> {
            self.0.fetch_add(1, Ordering::AcqRel);
            Ok(())
        }
    }

    struct FakeWaiter;

    impl WaitHandle for FakeWaiter {
        fn wait(&mut self) -> Result<PtyExit, PtyError> {
            std::thread::sleep(Duration::from_millis(100));
            Ok(PtyExit {
                exit_code: 0,
                signal: None,
            })
        }
    }

    impl PtyAdapter for FakeAdapter {
        fn spawn(
            &self,
            _session_id: Uuid,
            _profile: &ShellProfile,
            _size: PtySize,
        ) -> Result<SpawnedPty, PtyError> {
            Ok(SpawnedPty {
                reader: Box::new(Cursor::new(b"hostile:\x1b]52;c;secret\x07".to_vec())),
                writer: Box::new(FakeWriter(Arc::clone(&self.writes))),
                resize: Box::new(FakeResize(Arc::clone(&self.resized_rows))),
                killer: Box::new(FakeKiller(Arc::clone(&self.kills))),
                waiter: Box::new(FakeWaiter),
                process_id: Some(42),
            })
        }
    }

    fn fixture_profile() -> ShellProfile {
        #[cfg(unix)]
        let program = PathBuf::from("/bin/sh");
        #[cfg(windows)]
        let program =
            PathBuf::from(std::env::var_os("ComSpec").expect("Windows command interpreter"));
        ShellProfile {
            id: "fixture".into(),
            display_name: "Fixture".into(),
            program,
            arguments: Vec::new(),
            working_directory: None,
        }
    }

    fn fixture_size() -> PtySize {
        PtySize {
            rows: 24,
            columns: 80,
            pixel_width: 640,
            pixel_height: 480,
        }
    }

    #[test]
    fn fake_session_preserves_hostile_bytes_and_requires_credit() {
        let adapter = FakeAdapter::default();
        let session =
            LocalPtySession::start(&adapter, &fixture_profile(), fixture_size()).expect("session");
        assert!(session.receive_output(Duration::from_millis(10)).is_err());
        session.grant_output_credit(64).expect("credit");
        assert_eq!(
            session
                .receive_output(Duration::from_secs(1))
                .expect("output")
                .bytes,
            b"hostile:\x1b]52;c;secret\x07"
        );
    }

    #[test]
    fn input_sequence_and_resize_are_validated() {
        let adapter = FakeAdapter::default();
        let session =
            LocalPtySession::start(&adapter, &fixture_profile(), fixture_size()).expect("session");
        assert_eq!(
            session.send_input(1, b"wrong".to_vec()),
            Err(PtyError::InputSequence)
        );
        session.send_input(0, b"first".to_vec()).expect("input");
        session.send_input(1, b"second".to_vec()).expect("input");
        session
            .resize(PtySize {
                rows: 40,
                ..fixture_size()
            })
            .expect("resize");
        assert_eq!(adapter.resized_rows.load(Ordering::Acquire), 40);
    }

    #[test]
    fn limits_reject_invalid_dimensions_input_and_credit() {
        assert_eq!(
            PtySize {
                rows: 0,
                ..fixture_size()
            }
            .validate(),
            Err(PtyError::InvalidDimensions)
        );
        let adapter = FakeAdapter::default();
        let session =
            LocalPtySession::start(&adapter, &fixture_profile(), fixture_size()).expect("session");
        assert_eq!(session.grant_output_credit(0), Err(PtyError::InvalidCredit));
        assert_eq!(
            session.send_input(0, vec![0; MAX_INPUT_FRAME_BYTES + 1]),
            Err(PtyError::InvalidInput)
        );
    }

    #[test]
    fn byte_pressure_limits_are_enforced() {
        let flow = OutputFlow {
            state: Mutex::new(OutputFlowState::default()),
            changed: Condvar::new(),
        };
        flow.grant(MAX_OUTPUT_CREDIT_BYTES).expect("maximum credit");
        assert_eq!(flow.grant(1), Err(PtyError::InvalidCredit));

        let adapter = FakeAdapter::default();
        let session =
            LocalPtySession::start(&adapter, &fixture_profile(), fixture_size()).expect("session");
        session
            .pending_input
            .store(MAX_PENDING_INPUT_BYTES, Ordering::Release);
        assert_eq!(
            session.send_input(0, vec![b'x']),
            Err(PtyError::InputBackpressure)
        );
    }

    #[test]
    fn shell_discovery_returns_only_valid_absolute_executables() {
        let profiles = discover_shell_profiles();
        assert!(!profiles.is_empty());
        assert!(
            profiles.into_iter().all(
                |profile| profile.program.is_absolute() && is_executable_file(&profile.program)
            )
        );
    }
}
