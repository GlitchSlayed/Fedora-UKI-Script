use thiserror::Error;

/// Domain-specific command execution failures.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("command `{cmd}` failed with exit code {code:?}: {stderr}")]
    CommandFailed {
        cmd: String,
        code: Option<i32>,
        stderr: String,
    },

    #[error("root privileges required: run with sudo or as root")]
    RootRequired,
}
