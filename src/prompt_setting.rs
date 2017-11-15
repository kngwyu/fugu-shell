use yansi::Color;
use std::io::{self, Write};
use std::str::FromStr;
pub struct PromptSetting<'a> {
    face1: &'a str,
    face2: &'a str,
    color1: Color,
    color2: Color,
    color3: Color,
    dir_depth: usize,
}

impl<'a> PromptSetting<'a> {
    pub fn default() -> PromptSetting<'a> {
        PromptSetting {
            face1: "Fugu(°)#))<< ~",
            face2: "$ ",
            color1: Color::Cyan,
            color2: Color::Green,
            color3: Color::White,
            dir_depth: 2,
        }
    }
    pub fn print_face(&self, path: &String) {
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
        print!(
            "{}{}{}",
            self.color1.paint(self.face1),
            self.color2.paint(p),
            self.color3.paint(self.face2)
        );
        io::stdout().flush().unwrap();
    }
}
