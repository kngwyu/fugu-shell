extern crate walkdir;
extern crate termion;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate sloggers;
extern crate regex;

mod prompt_setting;
mod exec;
mod builtin;
mod common;
mod editor;
mod parser;
mod fugu_env;

use fugu_env::{FuguEnv, CommandType};
use prompt_setting::PromptSetting;
use exec::CommandList;
use editor::{Editor, Point, EditResult};
use parser::{Parser, ParseStatus};
use common::LOGGER;
use std::io::{stdin, stdout, Write};
use std::env;
use std::ops::Range;
use std::cmp::min;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use termion::clear;
use termion::style;
fn main() {
    let stdin = stdin();
    let screen = stdout();
    let mut screen = screen.lock().into_raw_mode().unwrap();
    screen.init_msg();
    let prompt = PromptSetting::default();
    let num = screen.reset_scr(2, &prompt);
    let mut editor = Editor::new(Point::new(num, 2));
    enum UiMode {
        Editor,
        Selector,
    }
    let mut parser = Parser::new();
    let mut selector = Selector::empty();
    let mut fuguenv = FuguEnv::new();
    for event in stdin.events() {
        let evt = event.unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Ctrl('p') | Key::Up => {
                        selector.csr_up();
                        screen.print_selector(&selector, &fuguenv, &editor);
                    }
                    Key::Ctrl('n') | Key::Down => {
                        selector.csr_down();
                        screen.print_selector(&selector, &fuguenv, &editor);
                    }
                    Key::Char('\t') => {}
                    _ => {
                        let res = editor.handle_key(&key);
                        match res {
                            EditResult::JustTailAdd(c) => {
                                parser.read1(c);
                                screen.print_editor(&editor, 0..1);
                            }
                            EditResult::JustAdd => {
                                parser = Parser::new();
                                parser.add_str(&editor.to_str());
                                screen.print_editor(&editor, 0..1);
                            }
                            EditResult::Edited => {
                                parser = Parser::new();
                                parser.add_str(&editor.to_str());
                                screen.print_editor(&editor, 0..1);
                            }
                            EditResult::Moved => screen.move_csr(&editor),
                            EditResult::None => {}
                        };
                        match res {
                            EditResult::JustTailAdd(_) |
                            EditResult::JustAdd |
                            EditResult::Edited => {
                                match parser.parse_status {
                                    ParseStatus::WaitCommand => {
                                        fuguenv.reset_search();
                                        fuguenv.search_cmd(parser.get_cur_token());
                                        let v = fuguenv.search_to_vec();
                                        selector = Selector::new(v);
                                        screen.print_selector(&selector, &fuguenv, &editor);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

struct Selector {
    buf: Vec<(usize, CommandType)>,
    range: (usize, usize),
    max_print: usize,
    cursor: Option<usize>,
}
impl Selector {
    fn empty() -> Selector {
        Selector {
            max_print: 1,
            range: (0, 1),
            buf: Vec::new(),
            cursor: None,
        }
    }
    fn new(b: Vec<(usize, CommandType)>) -> Selector {
        let m = min(b.len(), 15);
        Selector {
            max_print: m,
            range: (0, m),
            buf: b,
            cursor: None,
        }
    }
    fn csr_down(&mut self) {
        if let Some(num) = self.cursor {
            if num + 1 >= self.buf.len() {
                return;
            }
            self.cursor = Some(num + 1);
            if num + 1 >= self.max_print {
                self.range.0 += 1;
                self.range.1 += 1;
            }
        } else {
            self.cursor = Some(0);
        }
    }
    fn csr_up(&mut self) {
        if let Some(num) = self.cursor {
            if num > 0 {
                self.cursor = Some(num - 1);
                if num >= self.max_print {
                    self.range.0 -= 1;
                    self.range.1 -= 1;
                }
            }
        }
    }
}
use std::error::Error;
trait FuguScreen {
    fn init_msg(&mut self);
    fn reset_scr(&mut self, u16, &PromptSetting) -> usize;
    fn move_csr(&mut self, &Editor);
    fn print_editor(&mut self, &Editor, Range<usize>);
    fn print_selector(&mut self, &Selector, &FuguEnv, &Editor);
}
impl<W: Write> FuguScreen for W {
    fn init_msg(&mut self) {
        match write!(
        self,
        "{}{}Wellcome to Fugu Shell! (°)#))<< {}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Goto(1, 2),
    ) {
            Ok(_) => {}
            Err(why) => error!(LOGGER, "error in write! macro, {:?}", why.description()),
        }
        self.flush().unwrap();
    }

    fn reset_scr(&mut self, cur_y: u16, prompt: &PromptSetting) -> usize {
        let current_dir = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        match write!(self, "{}", cursor::Goto(1, cur_y)) {
            Ok(_) => {}
            Err(why) => error!(LOGGER, "error in write! macro, {:?}", why.description()),
        };
        prompt.print_face(&current_dir, self)
    }

    fn move_csr(&mut self, e: &Editor) {
        match write!(self, "{}", (e.cursor_base + e.cursor_buf).goto()) {
            Ok(_) => {}
            Err(why) => error!(LOGGER, "error in write! macro, {:?}", why.description()),  
        };
        self.flush().unwrap();
    }

    fn print_editor(&mut self, e: &Editor, range: Range<usize>) {
        for i in range {
            let cur_y = e.cursor_base.y + i;
            let pos = Point::new(e.cursor_base.x, cur_y);
            match write!(self, "{}{}{}", pos.goto(), clear::UntilNewline, e.buffer[i]) {
                Ok(_) => {}
                Err(why) => error!(LOGGER, "error in write! macro, {:?}", why.description()),
            }
        }
        self.move_csr(e);
    }
    fn print_selector(&mut self, sel: &Selector, env: &FuguEnv, edit: &Editor) {
        write!(self, "{}{}", cursor::Goto(1, 3), clear::AfterCursor).unwrap();
        for (j, i) in (sel.range.0..sel.range.1).enumerate() {
            let st = match sel.buf[i].1 {
                CommandType::Path => &env.path_cmds[sel.buf[i].0],
                CommandType::Builtin => env.builtin_cmds[sel.buf[i].0],
                _ => return,
            };
            let s = if let Some(k) = sel.cursor {
                if k == i {
                    format!(
                        "{}{}{}{}{}",
                        cursor::Goto(1, (j + 3) as u16),
                        clear::AfterCursor,
                        style::Underline,
                        st,
                        style::NoUnderline
                    )
                } else {
                    format!(
                        "{}{}{}",
                        cursor::Goto(1, (j + 3) as u16),
                        clear::AfterCursor,
                        st
                    )
                }
            } else {
                format!(
                    "{}{}{}",
                    cursor::Goto(1, (j + 3) as u16),
                    clear::AfterCursor,
                    st
                )
            };
            match write!(self, "{}", s) {
                Ok(_) => {}
                Err(why) => error!(LOGGER, "error in write! macro, {:?}", why.description()),
            }
        }
        self.move_csr(&edit);
    }
}
