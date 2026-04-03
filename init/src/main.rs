use std::fs;

use std::process::Command;

fn main() {
    for cmd in [
        "ls", "cat", "cp", "mv", "rm", "mkdir", "chmod", "echo", "grep", "mount", "umount",
        "clear", "vi", "head", "tail", "wc", "sort", "find", "xargs", "wget", "env", "export",
        "ps", "kill", "sleep", "uname", "id", "whoami", "sh", "ifconfig", "udhcpc", "route",
    ] {
        std::os::unix::fs::symlink("/bin/busybox", format!("/bin/{}", cmd)).ok();
    }

    fs::create_dir_all("/proc").ok();
    fs::create_dir_all("/sys").ok();
    fs::create_dir_all("/dev").ok();
    fs::create_dir_all("/tmp").ok();
    fs::create_dir_all("/etc").ok();

    for dir in ["/proc", "/sys", "/dev", "/tmp"] {
        println!("{} exists: {}", dir, std::path::Path::new(dir).exists());
    }

    Command::new("/bin/busybox")
        .args(["mount", "-t", "proc", "proc", "/proc"])
        .status()
        .ok();

    Command::new("/bin/busybox")
        .args(["mount", "-t", "sysfs", "sys", "/sys"])
        .status()
        .ok();

    Command::new("/bin/busybox")
        .args(["mount", "-t", "devtmpfs", "dev", "/dev"])
        .status()
        .ok();

    Command::new("/bin/busybox")
        .args(["ifconfig", "eth0", "up"])
        .status()
        .ok();
    Command::new("/bin/busybox")
        .args(["udhcpc", "-i", "eth0"])
        .status()
        .ok();

    println!("Welcome to izacos");

    loop {
        match Command::new("/bin/ash").spawn() {
            Ok(mut child) => match child.wait() {
                Ok(status) => {
                    println!("shell exited with status: {}", status);
                }
                Err(err) => {
                    eprintln!("init: failed waiting on shell: {}", err);
                }
            },
            Err(err) => {
                eprintln!("init: failed to start shell: {}", err);
            }
        }
    }
}
