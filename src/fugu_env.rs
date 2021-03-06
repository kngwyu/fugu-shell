use std::env;
use walkdir::WalkDir;
use builtin::*;
use common::{is_file_executable, LOGGER};
use regex::Regex;
use std::error::Error;
pub struct FuguEnv<'a> {
    pub path_cmds: Vec<String>,
    pub builtin_cmds: Vec<&'a str>, // ビルトイン関数
    fugu_vars: Vec<String>,         // Fugu変数
    env_vars: Vec<String>,          // 環境変数
    path_cache: Vec<bool>,
    builtin_cache: Vec<bool>,
}
impl<'a> FuguEnv<'a> {
    // todo: エラーハンドリングの追加
    // ハンドルするほどのエラーがあるかよくわからないが...
    // (だってディレクトリやファイルがなかったらどうしようもないからなあ)
    pub fn new() -> FuguEnv<'a> {
        let mut path_cmds = Vec::new();
        if let Some(paths) = env::var_os("PATH") {
            for path in env::split_paths(&paths) {
                let dirname = path.to_str().unwrap().to_owned();
                for entry in WalkDir::new(&dirname).min_depth(1).max_depth(1) {
                    let e = ok_or_continue!(entry);
                    let fname = e.file_name().to_os_string().into_string();
                    let fname = ok_or_continue!(fname);
                    let fdata = ok_or_continue!(e.metadata());
                    if fdata.is_file() && is_file_executable(&fdata) {
                        path_cmds.push(fname);
                    }
                }
            }
        }
        let builtin_cmds: Vec<&'a str> = BUILTIN_CMD.iter().cloned().collect();
        FuguEnv {
            path_cache: vec![true; path_cmds.len()],
            builtin_cache: vec![true; builtin_cmds.len()],
            path_cmds: path_cmds,
            builtin_cmds: builtin_cmds,
            fugu_vars: Vec::new(),
            env_vars: Vec::new(),
        }
    }
    pub fn reset_search(&mut self) {
        self.path_cache
            .iter_mut()
            .chain(self.builtin_cache.iter_mut())
            .for_each(|ref_v| *ref_v = true);
    }
    // TODO: これはハンドルしないとSyntax Highlightingできないので絶対必要
    pub fn search_cmd(&mut self, search_str: &str) {
        let re = match Regex::new(&search_str) {
            Ok(r) => r,
            Err(why) => {
                info!(LOGGER, "Regex Compile Failed, {:?}", why.description());
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
    pub fn get_cmd_str(&self, id: (usize, CommandType)) -> Option<&str> {
        match id.1 {
            CommandType::Path => Some(&self.path_cmds[id.0]),
            CommandType::Builtin => Some(self.builtin_cmds[id.0]),
            CommandType::User => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CommandType {
    Path,
    Builtin,
    User,
}
