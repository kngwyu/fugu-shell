#[macro_use]
extern crate log;
extern crate env_logger;
extern crate yansi;
extern crate walkdir;

mod read_line;
use read_line::read_cmd;
mod prompt_setting;
use prompt_setting::PromptSetting;
mod exec;
use exec::{CommandList, parse_cmd};
mod builtin;

use std::env;
fn main() {
    let _ = env_logger::init();
    main_loop();
}


fn main_loop() {
    let mut prompt_setting = PromptSetting::default();
    let mut before_dir = String::new();
    let mut cmd_list = CommandList::new();
    loop {
        let current_dir = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        prompt_setting.print_face(&current_dir);
        if before_dir != current_dir {
            cmd_list.upd_wd_commands(&current_dir);
            before_dir = current_dir;
        }
        let s = read_cmd();
        cmd_list.execute_command(parse_cmd(&s));
    }
}
