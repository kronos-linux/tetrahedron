pub use crate::error::*;

pub type Result<T> = core::result::Result<T, Error>;

pub use noshell::ShellCommand;

/// Run a shell command and stop the installation if there is an error
pub fn shrun(cmd: &ShellCommand) -> String {
    let cmd_name = String::from(cmd.command());

    match cmd.run() {
        Ok(s) => s,
        Err(e) => {
            let estr = "Shell command exited with a non-zero exit code".into();
            Error::NoShell(estr, cmd_name, e).handle()
        }
    }
}
