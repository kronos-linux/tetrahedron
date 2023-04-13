use clap::Parser;
use prelude::*;

mod assembly;
mod collect;
mod construct;
mod error;
mod prelude;
mod specfile;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Construst an initramfs directory instead of just creating a specfile
    #[arg(short, long)]
    construct: bool,

    /// Specify the path of the initramfs directory. Implies -c
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

    let asm_target = if let Some(t) = args.assembly {
        t
    } else {
        "/usr/src/assembly".into()
    };

    assembly::emerge_irfs(&asm_target);
    collect::dependencies(&asm_target);

    let spec_target = if let Some(t) = args.output {
        t
    } else {
        "/usr/src/initramfs.spec".into()
    };

    specfile::create(&spec_target, "/bins.txt", &asm_target);

    match (args.construct, args.directory) {
        (_, Some(d)) => construct::initramfs_dir(&d, &spec_target),
        (true, None) => construct::initramfs_dir("/usr/src/initramfs", &spec_target),
        (false, None) => (),
    }

    if !args.keep {
        shrun(&ShellCommand::new("rm").args(["-rf", &asm_target]));
    }

    Ok(())
}
