use std::collections::HashSet;
use std::env;
use std::process::Command;
use walkdir::WalkDir;
use builtin::*;
use common::LOGGER;

#[derive(Clone)]
pub struct CommandStore {
    pub name: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub pipe_in: bool,
    pub pipe_out: bool,
    pub wait: bool,
}
impl CommandStore {
    pub fn new() -> CommandStore {
        CommandStore {
            name: String::new(),
            args: Vec::new(),
            stdin: None,
            stdout: None,
            stderr: None,
            pipe_in: false,
            pipe_out: false,
            wait: true,
        }
    }
    pub fn add_name(&mut self, s: &str) {
        self.name = s.to_owned();
    }
    pub fn add_arg(&mut self, s: &str) {
        self.args.push(s.to_owned())
    }
    pub fn add_stdin(&mut self, s: &str) {
        self.stdin = Some(s.to_owned());
    }
    pub fn add_stdout(&mut self, s: &str) {
        self.stdout = Some(s.to_owned());
    }
    pub fn add_stderr(&mut self, s: &str) {
        self.stderr = Some(s.to_owned());
    }
}



// 'aはstaticのみ
pub struct CommandList<'a> {
    commands_in_path: HashSet<String>,
    commands_in_wd: HashSet<String>,
    commands_builtin: HashSet<&'a str>,
}
// Permissionから実行可能かどうか判定できる?
impl<'a> CommandList<'a> {
    // PATHのコマンドは一回しかsetしない
    pub fn new() -> CommandList<'a> {
        let mut path_cmds = HashSet::new();
        let s = "";
        match env::var_os("PATH") {
            Some(paths) => {
                for path in env::split_paths(&paths) {
                    let dirname = path.to_str().unwrap().to_owned();
                    for entry in WalkDir::new(&dirname).min_depth(1).max_depth(1) {
                        let e = entry.ok().unwrap();
                        let fname = match e.file_name().to_os_string().into_string() {
                            Ok(s) => s,
                            Err(_) => panic!("Error in into_string"),
                        };
                        let fdata = e.metadata().ok().unwrap();
                        if fdata.is_file() {
                            path_cmds.insert(fname);
                        }
                    }
                }
            }
            None => {}
        }
        let bulitin_cmds: HashSet<&str> = BUILTIN_CMD.iter().cloned().collect();
        CommandList {
            commands_in_path: path_cmds,
            commands_in_wd: HashSet::new(),
            commands_builtin: bulitin_cmds,
        }
    }
    pub fn upd_wd_commands(&mut self, wd: &str) {
        let mut wd_cmd = HashSet::new();
        for entry in WalkDir::new(wd).min_depth(1) {
            let e = match entry {
                Ok(e) => e,
                Err(err) => {
                    error!(LOGGER, "error in upd_wd, {:?}", err);
                    break;
                }
            };
            let fpath = e.path().to_str().unwrap().to_owned();
            let fdata = e.metadata().ok().unwrap();
            if fdata.is_file() {
                wd_cmd.insert(fpath);
            }
        }
        self.commands_in_wd = wd_cmd;
    }
    pub fn execute_command(&self, cmds: Vec<CommandStore>) {
        for storecm in cmds {
            let name = &*storecm.name;
            if self.commands_builtin.contains(name) {
                exec_builtin(&storecm);
                continue;
            }
            let ok = self.commands_in_path.contains(name) || self.commands_in_wd.contains(name);
            if ok {
                let mut cmd = Command::new(name);
                if !storecm.args.is_empty() {
                    cmd.args(&storecm.args);
                }
                cmd.spawn().expect("failed to execute process");
            } else {
                println!("Fugu: Unknown command '{}'", name)
            }
        }
    }
}
