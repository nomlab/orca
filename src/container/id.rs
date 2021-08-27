use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use nix::unistd::{geteuid, getegid};

pub struct MappingId {
    source_id: usize,
    target_id: usize,
    range: usize,
}

pub enum IdType {
    UID,
    GID,
    SUBUID,
    SUBGID,
}

impl MappingId {
    pub fn new(idtype: IdType) -> Self {
        match idtype {
            IdType::UID => {
                let uid = Self::get_uid();
                MappingId{
                    source_id: uid.0,
                    target_id: 0,
                    range: uid.1
                }
            }
            IdType::GID => {
                let gid = Self::get_gid();
                MappingId{
                    source_id: gid.0,
                    target_id: 0,
                    range: gid.1
                }
            }
            IdType::SUBUID => {
                let subuid = Self::get_subuid();
                MappingId{
                    source_id: subuid.0,
                    target_id: 1,
                    range: subuid.1
                }
            }
            IdType::SUBGID => {
                let subgid = Self::get_subgid();
                MappingId{
                    source_id: subgid.0,
                    target_id: 1,
                    range: subgid.1
                }
            }
        }
    }

    pub fn into_vec(&self) -> Vec<String> {
        vec![
            self.target_id.to_string(),
            self.source_id.to_string(),
            self.range.to_string()
        ]
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.target_id, self.source_id, self.range)
    }

    fn get_uid() -> (usize, usize) {
        let uid = geteuid().as_raw() as usize;
        let range = 1;
        (uid, range)
    }

    fn get_gid() -> (usize, usize) {
        let gid = getegid().as_raw() as usize;
        let range = 1;
        (gid, range)
    }

    fn get_subuid() -> (usize, usize) {
        let path = "/etc/subuid";
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        let _ = reader.read_to_string(&mut contents);

        let username = env::var("USER").unwrap();
        let user_entry = username + ":";

        let mut subuid: usize = 0;
        let mut range: usize = 0;

        for line in contents.lines() {
            if line.starts_with(&user_entry) {
                let v: Vec<&str> = line.split(':').collect();
                subuid = v[1].parse().unwrap();
                range = v[2].parse().unwrap();
            }
        }
        (subuid, range)
    }

    fn get_subgid() -> (usize, usize) {
        let path = "/etc/subgid";
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        let _ = reader.read_to_string(&mut contents);

        let username = env::var("USER").unwrap();
        let user_entry = username + ":";

        let mut subgid: usize = 0;
        let mut range: usize = 0;

        for line in contents.lines() {
            if line.starts_with(&user_entry) {
                let v: Vec<&str> = line.split(':').collect();
                subgid = v[1].parse().unwrap();
                range = v[2].parse().unwrap();
            }
        }
        (subgid, range)
    }
}
