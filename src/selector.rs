use fugu_env::CommandType;
use std::cmp::min;
pub struct Selector {
    pub buf: Vec<(usize, CommandType)>,
    pub range: (usize, usize),
    pub max_print: usize,
    pub cursor: Option<usize>,
}

impl Default for Selector {
    fn default() -> Selector {
         Selector {
            max_print: 1,
            range: (0, 1),
            buf: Vec::new(),
            cursor: None,
        }
    }
}

impl Selector {
    pub fn new(b: Vec<(usize, CommandType)>) -> Selector {
        let m = min(b.len(), 15);
        Selector {
            max_print: m,
            range: (0, m),
            buf: b,
            cursor: None,
        }
    }
    pub fn csr_down(&mut self) {
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
    pub fn csr_up(&mut self) {
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
