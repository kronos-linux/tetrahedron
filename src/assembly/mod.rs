use crate::prelude::*;

pub fn emerge_irfs(assembly_target: &str) {
    // rm -rf ff && mkdir ff && eselect profile set 1 && USE="" emerge -avnq1 --keep-going=y --ignore-world=y --deep --root=ff busybox && eselect profile set 9
    shrun(&ShellCommand::new("rm").args(["-rf", assembly_target]));
    shrun(&ShellCommand::new("mkdir").args(["-p", assembly_target]));

    let profile = get_profile();
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
