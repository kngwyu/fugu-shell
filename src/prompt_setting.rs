
use std::io::{self, Write};
use std::str::FromStr;
use termion::color;
use termion::color::*;
pub struct PromptSetting<'a> {
    face1: &'a str,
    face2: &'a str,
    color1: Box<Color>,
    color2: Box<Color>,
    color3: Box<Color>,
    dir_depth: usize,
}

impl<'a> PromptSetting<'a> {
    pub fn default() -> PromptSetting<'a> {
        PromptSetting {
            face1: "Fugu(°)#))<< ~",
            face2: "$ ",
            color1: Box::new(Black),
            color2: Box::new(Black),
            color3: Box::new(Black),
            dir_depth: 2,
        }
    }
    pub fn print_face<W: Write>(&self, path: &String, mut stdout: &mut W) {
        let p = {
            let mut cur = path.len() - 1;
            let mut cnt = 0;
            let s = path.as_bytes();
            while cnt < self.dir_depth {
                if cur == 0 {
                    break;
                }
                cur -= 1;
                if s[cur] == b'/' {
                    cnt += 1;
                }
            }
            path[cur..path.len()].to_owned()
        };
        write!(
            stdout,
            "{}{}{}{}{}{}",
            color::Fg(Cyan),
            self.face1,
            color::Fg(Green),
            p,
            color::Fg(LightWhite),
            self.face2,
        );
        stdout.flush().unwrap();
    }
}
