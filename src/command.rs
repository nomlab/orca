use std::{process, env};
use std::path::Path;
use std::ffi::OsStr;
use std::cmp::PartialEq;

pub struct Command<T, S, U>
    where
        T: AsRef<OsStr> + AsRef<Path>,
        S: IntoIterator<Item = U> + PartialEq,
        U: AsRef<OsStr>,
{
    cmd_name: T,
    args: Option<S>,
}

impl <T, S, U>Command<T, S, U>
    where
        T: AsRef<OsStr> + AsRef<Path>,
        S: IntoIterator<Item = U> + PartialEq,
        U: AsRef<OsStr>,
{
    pub fn new(cmd_name: T, args: Option<S>) -> Self {
        Self {
            cmd_name,
            args,
        }
    }

    pub fn is_exist(&self) -> bool {
        let command_path =
            env::var_os("PATH").and_then(|paths| {
                env::split_paths(&paths).filter_map(|dir| {
                    let full_path = dir.join(&self.cmd_name);
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                }).next()
            });

        if command_path != None {
            true
        } else {
            false
        }
    }

    pub fn execute(self) -> std::result::Result<(), ()> {
        let mut command = process::Command::new(&self.cmd_name);
        if self.args != None {
            command.args(self.args.unwrap());
        }
        let res = command.output();

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
