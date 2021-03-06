use walkdir::WalkDir;
use std::env;
use common::LOGGER;
use exec::CommandStore;
pub const BUILTIN_CMD: [&'static str; 2] = ["cd", "exit"];

pub enum BuiltinHandle {
    Cd,
    Exit,
    Error,
}

pub fn exec_builtin(cmd: &CommandStore) -> BuiltinHandle {
    macro_rules! handle_builtin {
        ($f: ident, $res: expr) =>  ({
            use self::BuiltinHandle::*;
            // TODO: Add error handling (now expect $f returns())
            $f(cmd);
            $res
        })
    }
    match &*cmd.name {
        "cd" => handle_builtin!(builtin_cd, Cd),
        "exit" => handle_builtin!(builtin_exit, Exit),
        _ => {
            error!(LOGGER, "Invalid Builtin Command");
            BuiltinHandle::Error
        }
    }
}

fn builtin_exit(cmd: &CommandStore) {}

fn builtin_cd(cmd: &CommandStore) {
    match cmd.args.len() {
        val if val > 1 => println!("cd: too many args"),
        val if val == 1 => {
            if let Ok(dir) = WalkDir::new(&cmd.args[0])
                .max_depth(0)
                .into_iter()
                .next()
                .unwrap()
            {
                let fdata = dir.metadata().ok().unwrap();
                if fdata.is_dir() {
                    if !env::set_current_dir(&cmd.args[0]).is_ok() {
                        println!("cd: failed");
                    }
                } else {
                    println!("cd: '{}' is not a directory", cmd.args[0])
                }
            } else {
                println!("cd: '{}' does not exist", cmd.args[0])
            }
        }
        _ => match env::var("HOME") {
            Ok(dir_str) => {
                if !env::set_current_dir(&dir_str).is_ok() {
                    println!("cd: failed");
                }
            }
            Err(e) => println!("cd: No home dir, {}", e),
        },
    }
}
