use clap::Parser;
use noshell::ShellCommand;
use prelude::shrun;

mod error;
mod prelude;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Construst an initramfs directory instead of printing a specfile
    #[arg(short, long)]
    construct: bool,

    /// Specify the path of the initramfs directory
    #[arg(short, long)]
    directory: Option<String>,

    /// Specify the output of the specfile
    #[arg(short, long)]
    output: Option<String>,

    /// Location of the temporary directory for portage to use
    #[arg(short, long)]
    assembly: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!(
        "{}",
        shrun(&ShellCommand::new("echo").args(["Hello world"]))
    );
}
