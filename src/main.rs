#[macro_use]
extern crate log;
extern crate env_logger;
extern crate walkdir;
extern crate termion;
#[macro_use]
extern crate lazy_static;

mod prompt_setting;
mod exec;
mod builtin;
mod common;
mod editor;

use prompt_setting::PromptSetting;
use exec::{CommandList, parse_cmd};
use editor::{Editor, Point};
use std::io::{stdin, stdout, Read, Write};
use std::env;
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let _ = env_logger::init();
    let stdin = stdin();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let prompt = PromptSetting::default();
    write!(
        stdout,
        "{}{}Wellcome to Fugu Shell! (°)#))<< {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2),
    )
    .unwrap();
    stdout.flush().unwrap();
    let current_dir = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let num = prompt.print_face(&current_dir, &mut stdout);
    let mut editor = Editor::new(Point::new(num, 2), &mut stdout);
    for event in stdin.events() {
        let evt = event.unwrap();
        match evt {
            Event::Key(key) => {
                editor.handle_key(&key, &mut stdout);
                editor.debug(&mut stdout);
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    //     loop {
    //         let current_dir = env::current_dir()
    //             .unwrap()
    //             .into_os_string()
    //             .into_string()
    //             .unwrap();
    //         prompt_setting.print_face(&current_dir);
    //         if before_dir != current_dir {
    //             cmd_list.upd_wd_commands(&current_dir);
    //             before_dir = current_dir;
    //         }
    //         let s = read_cmd();
    //         cmd_list.execute_command(parse_cmd(&s));
    //     }
}
