mod child;
mod id;

use std::ffi::{CString, CStr};
use std::thread;
use std::fs::{File, OpenOptions};
use std::io::{stdout, stdin, Write};
use crate::command::Command;
use std::os::unix::io::AsRawFd;
use nix::unistd::{Pid, geteuid};
use nix::sys::wait::wait;
use nix::sched::{clone, CloneFlags, setns};
use libc::{grantpt, unlockpt};
use child::Child;
use id::{MappingId, IdType};
use crate::image::Image;
use crate::STACK_SIZE;
use retry::{retry, delay::Fixed};

pub struct Container {
    image: Image,
    child_pid: Pid,
}

impl Container {
    pub fn new(
        image: Image,
        command: String,
        netns_flag: bool
    ) -> Self {

        let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let cb = Box::new(|| Self::child_main(&command, &image.rootfs_path, &image.image_name));

        let mut flags = CloneFlags::empty();
        flags.insert(CloneFlags::CLONE_NEWUSER);
        flags.insert(CloneFlags::CLONE_NEWNS);
        flags.insert(CloneFlags::CLONE_NEWUTS);
        flags.insert(CloneFlags::CLONE_NEWPID);
        flags.insert(CloneFlags::CLONE_NEWIPC);
        if netns_flag {
            flags.insert(CloneFlags::CLONE_NEWNET);
        }
        let signals = Some(libc::SIGCHLD);

        let child_pid = clone(cb, stack, flags, signals).unwrap();

        Container{
            image,
            child_pid
        }
    }

    pub fn map_id(&self) -> std::result::Result<(), ()> {
        let mapping_uid = MappingId::new(IdType::UID);
        let mapping_gid = MappingId::new(IdType::GID);

        let uid_map_path = format!("/proc/{}/uid_map", self.child_pid);
        let gid_map_path = format!("/proc/{}/gid_map", self.child_pid);
        let setgroups_path = format!("/proc/{}/setgroups", self.child_pid);

        let mut uid_map_file = OpenOptions::new().append(true).open(&uid_map_path).unwrap();
        uid_map_file.write_all(&mapping_uid.to_string().into_bytes()).unwrap();
        let mut setgroups_file = OpenOptions::new().append(true).open(&setgroups_path).unwrap();
        setgroups_file.write_all(b"deny").unwrap();
        let mut gid_map_file = OpenOptions::new().append(true).open(&gid_map_path).unwrap();
        gid_map_file.write_all(&mapping_gid.to_string().into_bytes()).unwrap();

        Ok(())
    }

    pub fn map_id_with_subuid(&self) -> std::result::Result<(), ()> {

        let mut args_uidmap: Vec<String> = vec![self.child_pid.to_string()];
        let mut args_gidmap: Vec<String> = vec![self.child_pid.to_string()];

        let mapping_uid = MappingId::new(IdType::UID);
        let mapping_gid = MappingId::new(IdType::GID);
        let mapping_subuid = MappingId::new(IdType::SUBUID);
        let mapping_subgid = MappingId::new(IdType::SUBGID);

        let _ = args_uidmap.append(&mut mapping_uid.into_vec());
        let _ = args_gidmap.append(&mut mapping_gid.into_vec());
        let _ = args_uidmap.append(&mut mapping_subuid.into_vec());
        let _ = args_gidmap.append(&mut mapping_subgid.into_vec());

        Command::new("newuidmap", Some(args_uidmap)).execute().unwrap();
        Command::new("newgidmap", Some(args_gidmap)).execute().unwrap();

        Ok(())
    }

    pub fn connect_tty(&self) -> std::result::Result<(), ()> {
        Self::setns(self.child_pid).unwrap();

        let pty_master = retry(Fixed::from_millis(50).take(20), || {
            nix::fcntl::open(
                "/dev/pts/ptmx",
                nix::fcntl::OFlag::O_RDWR,
                nix::sys::stat::Mode::all()
            )
        }).unwrap();

        if unsafe{ grantpt(pty_master) } < 0 {
            return Err(())
        }
        if unsafe{ unlockpt(pty_master) } < 0 {
            return Err(())
        }

        thread::spawn(move || {
            let stdout = stdout().as_raw_fd();
            let mut s: [u8; 1] = [0; 1];
            loop {
                if let Err(_) = nix::unistd::read(pty_master, &mut s) {
                    return;
                }
                if let Err(_) = nix::unistd::write(stdout, &s) {
                    return;
                }
            }
        });

        thread::spawn(move || {
            let stdin = stdin().as_raw_fd();
            let mut s: [u8; 1] = [0; 1];
            loop {
                if let Err(_) = nix::unistd::read(stdin, &mut s) {
                    return;
                }
                if let Err(_) = nix::unistd::write(pty_master, &s) {
                    return;
                }
            }
        });

        Ok(())
    }

    pub fn wait(self) -> std::result::Result<Image, ()> {
        let _ = wait().unwrap();
        Ok(self.image)
    }

    fn child_main(command: &str, path_rootfs: &str, image_name: &str) -> isize {

        retry(Fixed::from_millis(50).take(20), || {
            let uid = geteuid().as_raw() as u32;
            match uid {
                0 => Ok(()),
                _ => Err(())
            }
        }).unwrap();

        let child = Child::new(path_rootfs.to_string());
        child.pivot_root().unwrap();
        child.mount_all().unwrap();
        child.sethostname(image_name).unwrap();
        child.connect_tty().unwrap();

        let command_cstring = CString::new(command).unwrap();
        let command_cstr = command_cstring.as_c_str();

        let mut argv: Vec<&CStr> = Vec::new();
        argv.push(command_cstr);

        let mut envp: Vec<&CStr> = Vec::new();
        envp.push(CStr::from_bytes_with_nul(b"SHELL=/bin/sh\0").unwrap());
        envp.push(CStr::from_bytes_with_nul(b"HOME=/root\0").unwrap());
        envp.push(CStr::from_bytes_with_nul(b"TERM=xterm\0").unwrap());
        envp.push(CStr::from_bytes_with_nul(b"PATH=/bin:/usr/bin:/sbin:/usr/sbin\0").unwrap());

        child.exec(command_cstr, &argv, &envp);

        return 0; // Unreachable but neccessary for clone
    }

    fn setns(child_pid: Pid) -> std::result::Result<(), ()> {
        let raw_child_pid = child_pid.as_raw() as isize;
        let userns_filename = format!("/proc/{}/ns/user", raw_child_pid);
        let mntns_filename = format!("/proc/{}/ns/mnt", raw_child_pid);
        let userns = File::open(&userns_filename).unwrap();
        let mntns = File::open(&mntns_filename).unwrap();
        let userns_fd = userns.as_raw_fd();
        let mntns_fd = mntns.as_raw_fd();

        setns(userns_fd, CloneFlags::CLONE_NEWUSER).unwrap();
        setns(mntns_fd, CloneFlags::CLONE_NEWNS).unwrap();

        Ok(())
    }
}
