use termion::event::Key;
use termion::cursor;
use termion::clear;
use termion::cursor::DetectCursorPos;
use std::io::Write;
use std::error::Error;
#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }
    pub fn goto(&self) -> cursor::Goto {
        cursor::Goto(self.x as u16, self.y as u16)
    }
}

pub struct Editor {
    buffer: Vec<String>,
    cursor: Point, // 1-indexed
    init_cursor: Point, // 1-indexed
    cursor_buf: Point, // 0-indexed
    yank: Option<String>,
}
pub enum EditResult {
    Edited,
    Moved,
    None,
}
impl Editor {
    pub fn new<W: Write>(init_pos: Point, stdout: &mut W) -> Editor {
        // init_posはprompt_settingに依存
        let _ = write!(stdout, "{}", init_pos.goto());
        let mut v = Vec::new();
        v.push(String::new());
        Editor {
            buffer: v,
            cursor: init_pos,
            init_cursor: init_pos,
            cursor_buf: Point::new(0, 0),
            yank: None,
        }
    }
    // エディターのデバッグ用
    pub fn debug<W: Write>(&self, stdout: &mut W) {
        write!(
            stdout,
            "{}{}{:?}{:?}{:?}{}",
            cursor::Goto(1, 24),
            clear::CurrentLine,
            self.cursor_buf,
            self.cursor,
            self.buffer,
            self.cursor.goto()
        )
        .unwrap();
    }
    pub fn handle_key<W: Write>(&mut self, key: &Key, stdout: &mut W) -> EditResult {
        match *key {
            Key::Char(c) => {
                // insert
                let current_len = self.buffer[self.cursor_buf.y].len();
                if self.cursor_buf.x == current_len {
                    self.buffer[self.cursor_buf.y].push(c);
                    match write!(stdout, "{}", c) {
                        Ok(_) => {}
                        Err(why) => error!("error in write char, {:?}", why.description()),
                    };
                    self.cursor_buf.x += 1;
                    self.cursor.x += 1;
                } else {
                    assert!(self.cursor_buf.x < current_len);
                    self.buffer[self.cursor_buf.y].insert(self.cursor_buf.x, c);
                    self.cursor.x += 1;
                    match write!(
                        stdout,
                        "{}{}{}",
                        clear::UntilNewline,
                        &self.buffer[self.cursor_buf.y][self.cursor_buf.x..current_len + 1],
                        self.cursor.goto()
                    ) {
                        Ok(_) => {}
                        Err(why) => error!("error in write char, {:?}", why.description()),
                    }
                    self.cursor_buf.x += 1;
                }
                EditResult::Edited
            }
            Key::Ctrl('h') | Key::Backspace => {
                // insert
                let current_len = self.buffer[self.cursor_buf.y].len();
                if current_len == 0 || self.cursor_buf.x == 0 {

                } else if self.cursor_buf.x == current_len {
                    self.buffer[self.cursor_buf.y].pop();
                    self.cursor_buf.x -= 1;
                    self.cursor.x -= 1;
                    match write!(stdout, "{}{}", self.cursor.goto(), clear::UntilNewline) {
                        Ok(_) => {}
                        Err(why) => panic!("error in write char, {:?}", why.description()),
                    };
                } else {
                    assert!(self.cursor_buf.x < current_len);
                    self.cursor.x -= 1;
                    self.cursor_buf.x -= 1;
                    self.buffer[self.cursor_buf.y].remove(self.cursor_buf.x);
                    let current_len = self.buffer[self.cursor_buf.y].len();
                    match write!(
                        stdout,
                        "{}{}{}",
                        self.cursor.goto(),
                        clear::UntilNewline,
                        &self.buffer[self.cursor_buf.y][self.cursor_buf.x..current_len]
                    ) {
                        Ok(_) => {}
                        Err(why) => panic!("error in write char, {:?}", why.description()),
                    }
                }
                EditResult::None
            }
            Key::Ctrl('f') | Key::Right => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                if self.cursor_buf.x >= current_len {
                    EditResult::None
                } else {
                    self.cursor.x += 1;
                    self.cursor_buf.x += 1;
                    match write!(stdout, "{}", self.cursor.goto()) {
                        Ok(_) => {}
                        Err(why) => panic!("error in write char, {:?}", why.description()),
                    }
                    EditResult::None
                }
            }
            Key::Ctrl('b') | Key::Left => {
                if self.cursor_buf.x == 0 {
                    EditResult::None
                } else {
                    self.cursor.x -= 1;
                    self.cursor_buf.x -= 1;
                    match write!(stdout, "{}", self.cursor.goto()) {
                        Ok(_) => {}
                        Err(why) => panic!("error in write char, {:?}", why.description()),
                    }
                    EditResult::None
                }
            }
            Key::Ctrl('d') | Key::Delete => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                if self.cursor_buf.x == current_len {
                    EditResult::None
                } else {
                    self.buffer[self.cursor_buf.y].remove(self.cursor_buf.x);
                    let current_len = self.buffer[self.cursor_buf.y].len();
                    match write!(
                        stdout,
                        "{}{}",
                        clear::UntilNewline,
                        &self.buffer[self.cursor_buf.y][self.cursor_buf.x..current_len]
                    ) {
                        Ok(_) => {}
                        Err(why) => panic!("error in write char, {:?}", why.description()),
                    }
                    EditResult::None
                }
            }
            Key::Ctrl('a') | Key::Home => {
                self.cursor_buf.x = 0;
                self.cursor.x = self.init_cursor.x;
                match write!(stdout, "{}", self.cursor.goto()) {
                    Ok(_) => {}
                    Err(why) => panic!("error in write char, {:?}", why.description()),
                }
                EditResult::None
            }
            Key::Ctrl('e') | Key::End => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                self.cursor_buf.x = current_len;
                self.cursor.x = self.init_cursor.x + current_len;
                match write!(stdout, "{}", self.cursor.goto()) {
                    Ok(_) => {}
                    Err(why) => panic!("error in write char, {:?}", why.description()),
                }
                EditResult::None
            }
            Key::Ctrl('k') => {
                let current_len = self.buffer[self.cursor_buf.y].len();
                self.yank = Some(
                    self.buffer[self.cursor_buf.y][self.cursor_buf.x..current_len]
                        .to_owned(),
                );
                self.buffer[self.cursor_buf.y].truncate(self.cursor_buf.x);
                match write!(stdout, "{}", clear::UntilNewline) {
                    Ok(_) => {}
                    Err(why) => panic!("error in write char, {:?}", why.description()),
                }
                EditResult::Edited
            }
            Key::Ctrl('y') => {
                if let Some(ref s) = self.yank {
                    self.buffer[self.cursor_buf.y].insert_str(self.cursor_buf.x, s);
                    let current_len = self.buffer[self.cursor_buf.y].len();
                    match write!(
                        stdout,
                        "{}{}{}",
                        clear::UntilNewline,
                        &self.buffer[self.cursor_buf.y][self.cursor_buf.x..current_len],
                        cursor::Goto((self.cursor.x + s.len()) as u16, self.cursor.y as u16),
                    ) {
                        Ok(_) => {}
                        Err(why) => panic!("error in write char, {:?}", why.description()),
                    }
                    self.cursor_buf.x += s.len();
                    self.cursor.x += s.len();
                } else {
                    return EditResult::None;
                }
                self.yank = None;
                EditResult::Edited
            }
            Key::Ctrl('p') | Key::Up => EditResult::None,
            Key::Ctrl('n') | Key::Down => EditResult::None,
            _ => EditResult::None,
        }
    }
    // パーサにわたす
    pub fn to_str(&self) -> String {
        self.buffer.iter().fold(
            String::new(),
            |acc, ref s| acc + &*s,
        )
    }
}
