#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate ascii;
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

#[macro_use]
mod common;
mod settings;
mod exec;
mod builtin;
mod editor;
mod parser;
mod fugu_env;
mod fugu_screen;
mod selector;

use fugu_screen::{FuguScreen, ScreenError};
use fugu_env::FuguEnv;
use settings::PromptSetting;
use editor::{EditResult, Editor, Point};
use parser::{ParseStatus, Parser, DELIMITERS};
use common::{LOGGER, MATCHES};
use exec::CommandList;
use selector::Selector;
use std::io::{stdin, stdout};
use std::error::Error;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use ascii::AsciiChar;
macro_rules! handle_scr_err {
    ($err: expr) => {
        if let Err(e) = $err {
            error!(LOGGER, "print screen error: {}", e);
        }
    }
}

// TODO: Add Error Handling
fn repl() {
    let stdin = stdin();
    let screen = stdout();
    let mut screen = screen.lock().into_raw_mode().unwrap();
    screen.init_msg();
    let prompt = PromptSetting::default();
    let num = match screen.reset_scr(2, &prompt) {
        Ok(u) => u,
        Err(why) => panic!("Init Error, {:?}", why.description()),
    };
    let mut editor = Editor::new(Point::new(num, 2));
    let mut parser = Parser::new();
    let mut selector = Selector::default();
    let mut fuguenv = FuguEnv::new();
    let mut cmd_list = CommandList::new();
    for event in stdin.events() {
        trace!(LOGGER, "event: {:?}", event);
        let evt = ok_or_continue!(event);
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
                Key::Char('\n') => {
                    if let Some(id) = selector.get_idx() {
                        editor.delete_untill(&DELIMITERS);
                        let cmd = some_or_continue!(fuguenv.get_cmd_str(id));
                        editor.insert_str(cmd);
                        parser.remove_cur_token();
                        parser.add_str(&editor.to_str());
                        screen.print_editor(&editor, 0..1);
                    }
                    let parsed = parser.enter();
                    trace!(LOGGER, "{}", parsed.len());
                    cmd_list.execute_command(parsed);
                }
                _ => {
                    let res = editor.handle_key(&key);
                    let res2 = match res {
                        EditResult::JustTailAdd(c) => {
                            parser.read1(c);
                            screen.print_editor(&editor, 0..1)
                        }
                        EditResult::JustAdd => {
                            parser = Parser::new();
                            parser.add_str(&editor.to_str());
                            screen.print_editor(&editor, 0..1)
                        }
                        EditResult::Edited => {
                            parser = Parser::new();
                            parser.add_str(&editor.to_str());
                            screen.print_editor(&editor, 0..1)
                        }
                        EditResult::Moved => screen.move_csr(&editor),
                        EditResult::None => Ok(()),
                    };
                    handle_scr_err!(res2);
                    let res2 = match res {
                        EditResult::JustTailAdd(_) | EditResult::JustAdd | EditResult::Edited => {
                            match parser.parse_status {
                                ParseStatus::WaitCommand => {
                                    fuguenv.reset_search();
                                    fuguenv.search_cmd(parser.get_cur_token());
                                    let v = fuguenv.search_to_vec();
                                    selector = Selector::new(v);
                                    screen.print_selector(&selector, &fuguenv, &editor)
                                }
                                _ => Ok(()),
                            }
                        }
                        _ => Ok(()),
                    };
                    handle_scr_err!(res2);
                }
            },
            _ => {}
        }
    }
}

fn main() {
    if let Some(_f) = MATCHES.value_of("EXEC_FILE") {
        // stub!!!!
        unimplemented!();
    } else {
        repl();
    }
}
