use clap::Parser;
use prelude::*;

mod assembly;
mod collect;
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

    /// Keep temporary initramfs directory
    #[arg(short, long)]
    keep: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let target = if let Some(t) = args.assembly {
        t
    } else {
        "/usr/src/assembly".into()
    };

    assembly::emerge_irfs(&target);
    collect::dependencies(&target);

    Ok(())
}
