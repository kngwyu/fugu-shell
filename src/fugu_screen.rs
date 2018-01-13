use std::io::{self, Write};
use std::error::Error;
use std::ops::Range;
use std::env;
use std::fmt::{self, Display, Formatter};
use settings::PromptSetting;
use editor::{Editor, Point};
use fugu_env::{CommandType, FuguEnv};
use selector::Selector;
use termion::clear;
use termion::cursor;
use termion::style;

#[derive(Debug)]
pub struct ScreenError(String);

impl Display for ScreenError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ScreenError {
    fn description(&self) -> &str {
        &self.0
    }
}

macro_rules! impl_from_error {
    ($type: path) => {
        impl From<$type> for ScreenError {
            fn from(why: $type) -> Self {
                ScreenError(why.description().to_owned())
            }
        }
    }
}

impl_from_error!(io::Error);

pub trait FuguScreen {
    fn init_msg(&mut self) -> Result<(), ScreenError>;
    fn reset_scr(&mut self, u16, &PromptSetting) -> Result<usize, ScreenError>;
    fn move_csr(&mut self, &Editor) -> Result<(), ScreenError>;
    fn print_editor(&mut self, &Editor, Range<usize>) -> Result<(), ScreenError>;
    fn print_selector(&mut self, &Selector, &FuguEnv, &Editor) -> Result<(), ScreenError>;
}
impl<W: Write> FuguScreen for W {
    fn init_msg(&mut self) -> Result<(), ScreenError> {
        write!(
            self,
            "{}{}Wellcome to Fugu Shell! (°)#))<< {}",
            clear::All,
            cursor::Goto(1, 1),
            cursor::Goto(1, 2),
        )?;
        Ok(self.flush()?)
    }

    fn reset_scr(&mut self, cur_y: u16, prompt: &PromptSetting) -> Result<usize, ScreenError> {
        let current_dir = env::current_dir()?
            .into_os_string()
            .into_string()
            .expect("Failed to convert OsString to String!");
        let _ = write!(self, "{}", cursor::Goto(1, cur_y))?;
        Ok(prompt.print_face(&current_dir, self)?)
    }

    fn move_csr(&mut self, e: &Editor) -> Result<(), ScreenError> {
        write!(self, "{}", (e.cursor_base + e.cursor_buf).goto())?;
        Ok(self.flush()?)
    }

    fn print_editor(&mut self, e: &Editor, range: Range<usize>) -> Result<(), ScreenError> {
        for i in range {
            let cur_y = e.cursor_base.y + i;
            let pos = Point::new(e.cursor_base.x, cur_y);
            write!(self, "{}{}{}", pos.goto(), clear::UntilNewline, e.buffer[i])?
        }
        self.move_csr(e);
        Ok(())
    }
    fn print_selector(
        &mut self,
        sel: &Selector,
        env: &FuguEnv,
        edit: &Editor,
    ) -> Result<(), ScreenError> {
        write!(self, "{}{}", cursor::Goto(1, 3), clear::AfterCursor)?;
        for (j, i) in (sel.range.0..sel.range.1).enumerate() {
            let st = match sel.buf[i].1 {
                CommandType::Path => &env.path_cmds[sel.buf[i].0],
                CommandType::Builtin => env.builtin_cmds[sel.buf[i].0],
                // stub!!!!
                CommandType::User => unimplemented!(),
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
            write!(self, "{}", s)?;
        }
        self.move_csr(&edit);
        Ok(())
    }
}
