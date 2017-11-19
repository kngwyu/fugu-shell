#[macro_use]
extern crate log;
extern crate env_logger;
extern crate walkdir;
extern crate termion;
#[macro_use]
extern crate lazy_static;

mod read_line;
mod prompt_setting;
mod exec;
mod builtin;
mod common;
mod editor;


use read_line::read_cmd;
use prompt_setting::PromptSetting;
use exec::{CommandList, parse_cmd};

use std::io::{stdin, stdout, Read, Write};
use std::env;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;

fn main() {
    let _ = env_logger::init();
    let stdin = stdin();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let prompt = PromptSetting::default();
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();
    let mut pos = termion::cursor::Goto(1, 1);
    for event in stdin.events() {
        let evt = event.unwrap();
        let current_dir = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        prompt.print_face(&current_dir, &mut stdout);
        match evt {
            Event::Key(k) => {
                match k {
                    Key::Char('q') => break,
                    Key::Char(c) => write!(stdout, "{}{}", pos, c).unwrap(),
                    _ => {}
                }
            }
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        pos = termion::cursor::Goto(x, y);
                        write!(stdout, "{}x", pos).unwrap();
                    }
                    _ => (),
                }
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
