use termion::terminal_size;
pub struct TermSize {
    width: usize,
    height: usize,
}

pub struct Cursor {
    x: usize,
    y: usize,
}
