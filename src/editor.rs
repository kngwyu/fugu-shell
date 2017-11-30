use termion::event::Key;
use termion::cursor;
use termion::clear;
use std::io::Write;
use std::str::from_utf8;
pub struct Editor {
    pub buffer: Vec<String>,
    pub cursor_base: Point, // 1-indexed
    pub cursor_buf: Point, // 0-indexed
    yank: Option<String>,
}
pub enum EditResult {
    JustTailAdd(Char),
    JustAdd,
    Edited,
    Moved,
    None,
}
impl Editor {
    pub fn new(init_pos: Point) -> Editor {
        // init_posはprompt_settingに依存
        let mut v = Vec::new();
        v.push(String::new());
        Editor {
            buffer: v,
            cursor_base: init_pos,
            cursor_buf: Point::new(0, 0),
            yank: None,
        }
    }
    //    pub fn reset(init_pos: Point) -> Editor {}
    // エディターのデバッグ用 将来的には消す
    pub fn debug<W: Write>(&self, stdout: &mut W) {
        write!(
            stdout,
            "{}{}{:?}{:?}{}{}",
            cursor::Goto(1, 24),
            clear::CurrentLine,
            self.cursor_buf,
            self.buffer,
            self.to_str(),
            (self.cursor_buf + self.cursor_base).goto()
        )
        .unwrap();
    }
    pub fn handle_key(&mut self, key: &Key) -> EditResult {
        match *key {
            Key::Char(c) => {
                // insert
                let current_len = self.buffer[self.cursor_buf.y].len();
                if self.cursor_buf.x == current_len {
                    self.buffer[self.cursor_buf.y].push(c);
                    self.cursor_buf.x += 1;
                    EditResult::JustTailAdd(c)
                } else {
                    assert!(self.cursor_buf.x < current_len);
                    self.buffer[self.cursor_buf.y].insert(self.cursor_buf.x, c);
                    self.cursor_buf.x += 1;
                    EditResult::JustAdd
                }
            }
            Key::Ctrl('h') | Key::Backspace => {
                // insert
                let current_len = self.buffer[self.cursor_buf.y].len();
                if current_len == 0 || self.cursor_buf.x == 0 {
                    return EditResult::None;
                } else if self.cursor_buf.x == current_len {
                    self.buffer[self.cursor_buf.y].pop();
                    self.cursor_buf.x -= 1;
                } else {
                    assert!(self.cursor_buf.x < current_len);
                    self.cursor_buf.x -= 1;
                    self.buffer[self.cursor_buf.y].remove(self.cursor_buf.x);
                    let current_len = self.buffer[self.cursor_buf.y].len();
                }
                EditResult::Edited
            }
            Key::Ctrl('f') | Key::Right => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                if self.cursor_buf.x >= current_len {
                    EditResult::None
                } else {
                    self.cursor_buf.x += 1;
                    EditResult::Moved
                }
            }
            Key::Ctrl('b') | Key::Left => {
                if self.cursor_buf.x == 0 {
                    EditResult::None
                } else {
                    self.cursor_buf.x -= 1;
                    EditResult::Moved
                }
            }
            Key::Ctrl('d') | Key::Delete => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                if self.cursor_buf.x == current_len {
                    EditResult::None
                } else {
                    self.buffer[self.cursor_buf.y].remove(self.cursor_buf.x);
                    let current_len = self.buffer[self.cursor_buf.y].len();
                    EditResult::Edited
                }
            }
            Key::Ctrl('a') | Key::Home => {
                self.cursor_buf.x = 0;
                EditResult::Moved
            }
            Key::Ctrl('e') | Key::End => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                self.cursor_buf.x = current_len;
                EditResult::Moved
            }
            Key::Ctrl('k') => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                self.yank = Some(
                    self.buffer[self.cursor_buf.y][self.cursor_buf.x..current_len]
                        .to_owned(),
                );
                self.buffer[self.cursor_buf.y].truncate(self.cursor_buf.x);
                EditResult::Edited
            }
            Key::Ctrl('y') => {
                if let Some(ref s) = self.yank {
                    self.buffer[self.cursor_buf.y].insert_str(self.cursor_buf.x, s);
                    let current_len = self.buffer[self.cursor_buf.y].len();
                    self.cursor_buf.x += s.len();
                } else {
                    return EditResult::None;
                }
                EditResult::JustAdd
            }

            _ => EditResult::None,
        }
    }
    // パーサにわたす
    pub fn to_str(&self) -> String {
        self.buffer.iter().fold(String::new(), |acc, ref s| {
            if let Some(&c) = s.as_bytes().last() {
                if c == b'\\' {
                    let bytes = s.as_bytes();
                    let len = bytes.len();
                    acc + from_utf8(&bytes[0..len - 1]).unwrap()
                } else {
                    acc + &*s
                }
            } else {
                acc + &*s
            }
        })
    }
}

use std::ops::{Add, Sub};
use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }
    pub fn goto(&self) -> cursor::Goto {
        cursor::Goto(self.x as u16, self.y as u16)
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        let xcmp = self.x.partial_cmp(&other.x).unwrap();
        match xcmp {
            Ordering::Equal => self.y.partial_cmp(&other.y).unwrap(),
            _ => xcmp,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Point {}
