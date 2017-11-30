use std::env;
use walkdir::WalkDir;
use builtin::*;
use common::{LOGGER, is_file_executable};
use regex::Regex;
use std::error::Error;
pub struct FuguEnv<'a> {
    pub path_cmds: Vec<String>,
    pub builtin_cmds: Vec<&'a str>, // ビルトイン関数
    fugu_vars: Vec<String>, // Fugu変数
    env_vars: Vec<String>, // 環境変数
    path_cache: Vec<bool>,
    builtin_cache: Vec<bool>,
}
impl<'a> FuguEnv<'a> {
    pub fn new() -> FuguEnv<'a> {
        let mut path_cmds = Vec::new();
        if let Some(paths) = env::var_os("PATH") {
            for path in env::split_paths(&paths) {
                let dirname = path.to_str().unwrap().to_owned();
                for entry in WalkDir::new(&dirname).min_depth(1).max_depth(1) {
                    let e = entry.ok().unwrap();
                    let fname = match e.file_name().to_os_string().into_string() {
                        Ok(s) => s,
                        Err(_) => {
                            error!(LOGGER, "Error in into_string");
                            continue;
                        }
                    };
                    let fdata = e.metadata().ok().unwrap();
                    if fdata.is_file() && is_file_executable(&fdata) {
                        path_cmds.push(fname);
                    }
                }
            }
        }
        let builtin_cmds: Vec<&'a str> = BUILTIN_CMD.iter().cloned().collect();
        FuguEnv {
            path_cache: vec![false; path_cmds.len()],
            builtin_cache: vec![false; builtin_cmds.len()],
            path_cmds: path_cmds,
            builtin_cmds: builtin_cmds,
            fugu_vars: Vec::new(),
            env_vars: Vec::new(),
        }
    }
    fn reset_search(&mut self) {
        for x in &mut self.path_cache {
            *x = false;
        }
        for x in &mut self.builtin_cache {
            *x = false;
        }
    }
    fn search_cmd(&mut self, search_str: &str) {
        let re = match Regex::new(&search_str) {
            Ok(r) => r,
            Err(why) => {
                debug!(LOGGER, "Regex Compile Failed, {:?}", why.description());
                return;
            }
        };
        for (i, s) in self.path_cmds.iter().enumerate() {
            if self.path_cache[i] == false {
                continue;
            }
            if !re.is_match(s) {
                self.path_cache[i] = false;
            }
        }
        for (i, s) in self.builtin_cmds.iter().enumerate() {
            if self.builtin_cache[i] == false {
                continue;
            }
            if !re.is_match(s) {
                self.builtin_cache[i] = false;
            }
        }
    }
    pub fn search_to_vec(&self) -> Vec<(usize, CommandType)> {
        let mut res = Vec::new();
        for (i, x) in self.path_cache.iter().enumerate() {
            if *x {
                res.push((i, CommandType::Path));
            }
        }
        for (i, x) in self.builtin_cache.iter().enumerate() {
            if *x {
                res.push((i, CommandType::Builtin));
            }
        }
        res
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CommandType {
    Path,
    Builtin,
    User,
}
