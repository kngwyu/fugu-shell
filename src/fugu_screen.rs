use std::io::Write;
use std::error::Error;
use std::ops::Range;
use std::env;
use settings::PromptSetting;
use editor::{Editor, Point};
use fugu_env::{CommandType, FuguEnv};
use selector::Selector;
use common::LOGGER;
use termion::clear;
use termion::cursor;
use termion::style;
pub trait FuguScreen {
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
