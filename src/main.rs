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
mod parser;
mod fugu_env;

use fugu_env::FuguEnv;
use prompt_setting::PromptSetting;
use exec::CommandList;
use editor::{Editor, Point, EditResult};
use std::io::{stdin, stdout, Read, Write};
use std::env;
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::ops::Range;
fn main() {
    let _ = env_logger::init();
    let _ = FuguEnv::new();
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
    let mut editor = Editor::new(Point::new(num, 2));
    for event in stdin.events() {
        let evt = event.unwrap();
        match evt {
            Event::Key(key) => {
                match editor.handle_key(&key) {
                    EditResult::Edited => print_scr(&editor, &mut stdout, 0..1),
                    EditResult::Moved => move_csr(&editor, &mut stdout),
                    EditResult::None => {}
                };
                editor.debug(&mut stdout);
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
use termion::cursor;
use termion::clear;
fn move_csr<W: Write>(e: &Editor, stdout: &mut W) {
    write!(stdout, "{}", (e.cursor_base + e.cursor_buf).goto()).unwrap();
}
fn print_scr<W: Write>(e: &Editor, stdout: &mut W, range: Range<usize>) {
    for i in range {
        let cur_y = e.cursor_base.y + i;
        let pos = Point::new(e.cursor_base.x, cur_y);
        write!(
            stdout,
            "{}{}{}",
            pos.goto(),
            clear::UntilNewline,
            e.buffer[i]
        )
        .unwrap();
    }
    move_csr(e, stdout);
}



fn exp() {
    use std::time::{Duration, SystemTime};
    let loopnum = 10000000;
    let char_num = 500;
    let a = SystemTime::now();
    let mut array = Vec::new();
    for i in 0..char_num {
        array.push(i + 1);
    }
    let mut k = 0;
    for i in 0..loopnum {
        k += i % array[i % char_num];
    }
    let b = SystemTime::now();
    println!("{:?}", b.duration_since(a).unwrap());
    let mut map = std::collections::BTreeMap::new();
    for i in 0..char_num {
        map.insert(i, i + 1);
    }
    for i in 0..loopnum {
        k += i % map[&(i % char_num)];
    }
    let c = SystemTime::now();
    println!("{:?}", c.duration_since(b).unwrap());
}
