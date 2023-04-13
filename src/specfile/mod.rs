use crate::prelude::*;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

static DELIM: &str = "/";
static PERMS: &str = "755 0 0";
static ROOT_DIRS: [&str; 11] = [
    "bin", "dev", "lib", "lib64", "mnt", "root", "proc", "sbin", "sys", "run", "usr",
];

pub fn create(specfile_target: &str, binfile: &str, assembly_dir: &str) {
    println!("Creating the specfile...");
    let files = get_lines(&(String::from(assembly_dir) + binfile));
    let files = agglomerate(&files);
    let mut rdirs = HashSet::new();
    for d in ROOT_DIRS {
        rdirs.insert(String::from(DELIM) + d);
    }

    let dirs = union(&[rdirs, decompose(&files)]);

    let lgcc: Vec<String> =
        shrun(&ShellCommand::new("find").args(["/usr", "-name", "libgcc*.so.[1-9]"]))
            .lines()
            .map(|x| x.to_string())
            .collect();
    let syslibs = agglomerate(&lgcc[..]);
    let sysdirs = decompose(&syslibs);

    let mut specfile = File::create(specfile_target).expect("Failed to open target specfile");

    compose(&mut specfile, assembly_dir, dirs, files);
    compose(&mut specfile, "", sysdirs, syslibs);
    println!("Specfile created");
}

fn agglomerate(strings: &[String]) -> HashSet<String> {
    let mut h = HashSet::new();

    for s in strings {
        h.insert(s.clone());
    }

    h
}

fn decompose(strings: &HashSet<String>) -> HashSet<String> {
    let mut h = HashSet::new();

    for item in strings {
        let sub_items = item.split(DELIM);
        let mut subdirs = Vec::new();
        for s in sub_items {
            if s == "" {
                continue;
            }

            subdirs.push(s)
        }

        for i in 1..subdirs.len() {
            let val = subdirs[0..i].join(DELIM);
            h.insert(String::from(DELIM) + &val);
        }
    }

    h
}

fn get_lines(target: &str) -> Vec<String> {
    let f = File::open(target).expect(&format!("Failed to open target file: {}", target));
    let b_reader = BufReader::new(f);
    b_reader
        .lines()
        .collect::<std::result::Result<_, _>>()
        .expect("Failed to read target file")
}

fn union(a: &[HashSet<String>]) -> HashSet<String> {
    let mut h = HashSet::new();
    for i in a {
        for j in i {
            h.insert(j.clone());
        }
    }
    h
}

fn compose(specfile: &mut File, a: &str, dirs: HashSet<String>, files: HashSet<String>) {
    for d in dirs {
        let w_str = format!("dir\t{}\t{}", d, PERMS);
        writeln!(specfile, "{}", w_str).expect("Failed to write to specfile");
    }
    for f in files {
        let w_str = format!("file\t{}\t{}{}\t{}", f, a, f, PERMS);
        writeln!(specfile, "{}", w_str).expect("Failed to write to specfile");
    }
}
