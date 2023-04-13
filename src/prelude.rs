use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

pub fn get_lines(target: &str) -> Vec<String> {
    let f = File::open(target).expect(&format!("Failed to open target file: {}", target));
    let b_reader = BufReader::new(f);
    b_reader
        .lines()
        .collect::<std::result::Result<_, _>>()
        .expect("Failed to read target file")
}
