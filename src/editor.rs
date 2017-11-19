use std::collections::LinkedList;
pub struct Cursor {
    x: usize,
    y: usize,
}
pub struct Editor {
    buffer: LinkedList<String>,
    cursor: Cursor,
    cursor_in_buf: Cursor,
}
