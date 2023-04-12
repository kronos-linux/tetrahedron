use std::process::{exit, Command, Stdio};

use crate::prelude::*;

pub fn emerge_irfs(assembly_target: &str) {
    shrun(&ShellCommand::new("rm").args(["-rf", assembly_target]));
    shrun(&ShellCommand::new("mkdir").args(["-p", assembly_target]));

    let cwd = std::env::current_dir().expect("Failed to find current direcotry");
    let profile = get_profile();

    std::env::set_current_dir(assembly_target).expect("Failed to change directory");
    shrun(&ShellCommand::new("mkdir").args(["dev"]));
    shrun(&ShellCommand::new("cp").args(["-a", "/dev/null", "dev"]));
    std::env::set_current_dir(&cwd).expect("Failed to change directory");

    shrun(&ShellCommand::new("eselect").args(["profile", "set", "1"]));

    println!("Emerging necessary programs and libraries for the initramfs...");
    match emerge_pkg(assembly_target, 0) {
        Err(_) => {
            println!("Failed to emerge initramfs 5 times. Aborting");
            exit(1);
        }
        _ => (),
    }
    println!("Emerge complete");

    shrun(&ShellCommand::new("eselect").args(["profile", "set", &profile.to_string()]));
}

fn emerge_pkg(t: &str, tri: u8) -> std::result::Result<(), ()> {
    if tri == 5 {
        return Err(());
    }

    if Command::new("emerge")
        .env("USE", "lvm gcrypt urandom -openssl")
        .args([
            "-vnq",
            "--keep-going=y",
            "--ignore-world=y",
            "--deep",
            &format!("--root={}", t),
            "sys-apps/busybox",
            "sys-fs/cryptsetup",
            "sys-fs/lvm2",
            "app-crypt/gnupg",
            "app-shells/bash",
            "app-misc/pax-utils",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Could not spawn emerge")
        .wait()
        .expect("Could not wait for emerge")
        .success()
    {
        Ok(())
    } else {
        emerge_pkg(t, tri + 1)
    }
}

fn get_profile() -> u8 {
    let profs = shrun(&ShellCommand::new("eselect").args(["profile", "list"]));
    let active = shrun(&ShellCommand::new("grep").pipe_string(profs).args(["\\*"]));
    let number = shrun(
        &ShellCommand::new("awk")
            .pipe_string(active)
            .args(["{print $1}"]),
    );
    let number = shrun(
        &ShellCommand::new("sed")
            .pipe_string(number)
            .args(["-e", "s|[^0-9]||g"]),
    );

    number
        .replace("\n", "")
        .parse()
        .expect("Failed to find a valid active profile")
}
