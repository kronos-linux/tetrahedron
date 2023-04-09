use std::process::{Command, Stdio};

use crate::prelude::*;

pub fn emerge_irfs(assembly_target: &str) {
    // rm -rf ff && mkdir ff && eselect profile set 1 && USE="" emerge -avnq1 --keep-going=y --ignore-world=y --deep --root=ff busybox && eselect profile set 9
    shrun(&ShellCommand::new("rm").args(["-rf", assembly_target]));
    shrun(&ShellCommand::new("mkdir").args(["-p", assembly_target]));

    let cwd = std::env::current_dir().expect("Failed to find current direcotry");
    let profile = get_profile();

    std::env::set_current_dir(assembly_target).expect("Failed to change directory");
    shrun(&ShellCommand::new("mkdir").args(["dev"]));
    shrun(&ShellCommand::new("cp").args(["-a", "/dev/null", "dev"]));
    std::env::set_current_dir(&cwd).expect("Failed to change directory");

    shrun(&ShellCommand::new("eselect").args(["profile", "set", "1"]));

    emerge_pkg(assembly_target);

    shrun(&ShellCommand::new("eselect").args(["profile", "set", &profile.to_string()]));
}

fn emerge_pkg(t: &str) {
    Command::new("emerge")
        .env("USE", "lvm kernel -openssl")
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
        .expect("Failed emerge of initramfs programs");
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
