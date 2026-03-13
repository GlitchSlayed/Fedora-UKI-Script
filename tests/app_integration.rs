use anyhow::Result;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ffi::OsStr;

#[path = "../src/app.rs"]
mod app;
#[path = "../src/cmd.rs"]
mod cmd;
#[path = "../src/config.rs"]
mod config;
#[path = "../src/errors.rs"]
mod errors;
#[path = "../src/fsops.rs"]
mod fsops;

use app::App;
use cmd::{CmdOutput, CommandRunner};
use config::Config;

struct MockRunner {
    replies: RefCell<VecDeque<CmdOutput>>,
}

impl MockRunner {
    fn new(replies: Vec<CmdOutput>) -> Self {
        Self {
            replies: RefCell::new(replies.into()),
        }
    }
}

impl CommandRunner for MockRunner {
    fn run<I, S>(&self, _program: &str, _args: I) -> Result<CmdOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.replies
            .borrow_mut()
            .pop_front()
            .ok_or_else(|| anyhow::anyhow!("no mock reply"))
    }
}

#[test]
fn list_backups_empty_ok() {
    let temp = tempfile::tempdir().unwrap_or_else(|e| panic!("{e}"));
    let mut cfg = Config::default();
    cfg.backup_root = temp.path().join("backups");
    let app = App::new(cfg, MockRunner::new(vec![]), true);
    let list = app.list_backups().unwrap_or_else(|e| panic!("{e}"));
    assert!(list.is_empty());
}

#[test]
fn sanitize_works() {
    assert_eq!(
        app::sanitize_cmdline("BOOT_IMAGE=x root=UUID=y rw"),
        "root=UUID=y rw"
    );
}
