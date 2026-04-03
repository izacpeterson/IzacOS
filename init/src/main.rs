use std::fs;
use std::os::unix::io::AsRawFd;
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
        .args(["udhcpc", "-i", "eth0", "-n", "-t", "3", "-T", "2"])
        .status()
        .ok();

    Command::new("/bin/busybox")
        .args(["hostname", "izacos"])
        .status()
        .ok();

    Command::new("/bin/busybox").args(["clear"]).status().ok();

    let motd = fs::read_to_string("/etc/motd").expect("Failed to read file");
    println!("{}", motd);

    unsafe {
        libc::setsid();
        let tty = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty1")
            .expect("Failed to open /dev/tty1");
        let fd = tty.as_raw_fd();
        libc::ioctl(fd, libc::TIOCSCTTY, 0);
        libc::dup2(fd, 0);
        libc::dup2(fd, 1);
        libc::dup2(fd, 2);
    }

    loop {
        match Command::new("/bin/zsh").env("TERM", "linux").spawn() {
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
