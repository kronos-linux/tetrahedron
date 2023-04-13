use crate::prelude::*;

#[derive(Debug)]
enum Type {
    Directory,
    File(String),
}

#[derive(Debug)]
struct Permissions {
    mode: String,
    uid: u16,
    gid: u16,
}

#[derive(Debug)]
struct Linespec {
    linetype: Type,
    dest: String,
    perms: Permissions,
}

pub fn initramfs_dir(dir: &str, specfile: &str) {
    let mut parsed_lines = Vec::new();
    let lines = get_lines(specfile);

    println!("Parsing initramfs specfile...");
    for line in lines {
        parsed_lines.push(parse_line(&line));
    }

    shrun(&ShellCommand::new("rm").args(["-rf", &dir]));
    shrun(&ShellCommand::new("mkdir").args(["-p", &dir]));

    for l in parsed_lines {
        match l.linetype {
            Type::Directory => create_dir(&(String::from(dir) + &l.dest), l.perms),
            Type::File(src) => copy_file(&src, &(String::from(dir) + &l.dest), l.perms),
        }
    }
}

fn create_dir(target: &str, perms: Permissions) {
    println!("Creating directory: {}", target);
    shrun(&ShellCommand::new("mkdir").args(["-p", target]));
    set_perms(target, perms);
}

fn copy_file(src: &str, dest: &str, perms: Permissions) {
    println!("Moving file: {} -> {}", src, dest);
    shrun(&ShellCommand::new("cp").args([src, dest]));
    set_perms(dest, perms);
}

fn set_perms(target: &str, perms: Permissions) {
    shrun(&ShellCommand::new("chmod").args([&perms.mode, target]));
    shrun(&ShellCommand::new("chown").args([&format!("{}:{}", perms.uid, perms.gid), target]));
}

fn parse_line(line: &str) -> Linespec {
    let line: Vec<&str> = line.split("\t").collect();

    if line[0] == "dir" {
        Linespec {
            linetype: Type::Directory,
            dest: String::from(line[1]),
            perms: Permissions::new(line[2]),
        }
    } else if line[0] == "file" {
        Linespec {
            linetype: Type::File(String::from(line[2])),
            dest: String::from(line[1]),
            perms: Permissions::new(line[3]),
        }
    } else {
        panic!()
    }
}

impl Permissions {
    fn new(spec: &str) -> Self {
        let p_vec: Vec<&str> = spec.split(" ").collect();
        let mode_vec: Vec<&str> = p_vec[0].split("").collect();
        let p: Vec<u16> = p_vec.iter().map(|x| x.parse().expect("aaa")).collect();

        Permissions {
            mode: mode_vec.join(""),
            uid: p[1],
            gid: p[2],
        }
    }
}
