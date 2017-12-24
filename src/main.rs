#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate slog;
extern crate sloggers;
extern crate termion;
extern crate walkdir;

mod settings;
mod exec;
mod builtin;
mod common;
mod editor;
mod parser;
mod fugu_env;
mod fugu_screen;
mod selector;

use fugu_screen::FuguScreen;
use fugu_env::FuguEnv;
use settings::PromptSetting;
use editor::{EditResult, Editor, Point};
use parser::{ParseStatus, Parser};
use common::{LOGGER, MATCHES};
use selector::Selector;
use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn repl() {
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
    let mut selector = Selector::default();
    let mut fuguenv = FuguEnv::new();
    for event in stdin.events() {
        let evt = event.unwrap();
        match evt {
            Event::Key(key) => match key {
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
                        EditResult::JustTailAdd(_) | EditResult::JustAdd | EditResult::Edited => {
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
            },
            _ => {}
        }
    }
}

fn main() {
    if let Some(f) = MATCHES.value_of("EXEC_FILE") {

    } else {
        repl();
    }
}
