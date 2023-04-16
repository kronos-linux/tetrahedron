use crate::prelude::*;
use std::{fs::File, io::Write};

pub fn dependencies(t: &str) {
    let th = format!("{}/hunter.sh", &t);

    {
        write!(
        File::create(&th).expect("Failed to create a hunter script"),
        "{}",
        "#!/bin/bash -e\n/bin/busybox --install -s\nlddtree -l \"$(which busybox)\" \"$(which cryptsetup)\" \"$(which gpg)\" \"$(which lvm)\" > /bins.txt\necho \"/init\" >> /bins.txt\n")
        .expect("Failed to create the hunter script")
    }

    shrun(&ShellCommand::new("chmod").args(["+x", &th]));
    println!("Collecting dependencies for busybox, cryptsetup, lvm, and gpg...");
    shrun(&ShellCommand::new("chroot").args([&t, "/hunter.sh"]));
    println!("Collection complete");
}
