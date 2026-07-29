#![forbid(unsafe_code)]
#[cfg(feature = "desktop-runtime")]
pub mod desktop_runtime;
pub mod launch_endpoint;
pub mod lifecycle;
pub mod local_metadata;
pub mod pty;
pub mod session_security;
pub mod single_instance;

pub mod ipc;
