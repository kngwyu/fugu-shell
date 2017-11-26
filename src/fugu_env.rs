// std::envと名前がかぶらないようにfugu_envにした
use std::env;
use std::process::Command;
use walkdir::WalkDir;
use builtin::*;
use std::os::unix::fs::MetadataExt;
use std::fs::Metadata;
pub struct FuguEnv<'a> {
    path_cmds: Vec<String>,
    builtin_cmds: Vec<&'a str>, // ビルトイン関数
    fugu_vars: Vec<String>, // Fugu変数
    env_vars: Vec<String>, // 環境変数
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
                        Err(_) => panic!("Error in into_string"),
                    };
                    let fdata = e.metadata().ok().unwrap();
                    if fdata.is_file() && is_file_executable(&fdata) {
                        path_cmds.push(fname);
                    }
                }
            }
        }
        println!("{}", path_cmds.len());
        let builtin_cmds: Vec<&'a str> = BUILTIN_CMD.iter().cloned().collect();
        FuguEnv {
            path_cmds: path_cmds,
            builtin_cmds: builtin_cmds,
            fugu_vars: Vec::new(),
            env_vars: Vec::new(),
        }
    }
}

fn is_file_executable(fdata: &Metadata) -> bool {
    (fdata.mode() & 0o111) != 0
}
