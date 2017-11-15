use std::io;
fn readline() -> Option<String> {
    let mut input_str = String::new();
    match io::stdin().read_line(&mut input_str) {
        Ok(_) => Some(input_str),
        Err(err) => {
            error!("Error in readline: {}", err);
            None
        }
    }
}
pub fn read_cmd() -> String {
    let mut res = String::new();
    while let Some(mut cmd_str) = readline() {
        cmd_str.pop();
        if cmd_str.is_empty() {
            break;
        } else if *cmd_str.as_bytes().last().unwrap() == b'\\' {
            cmd_str.pop();
            res.push_str(&cmd_str);
        } else {
            res.push_str(&cmd_str);
            break;
        }
    }
    res
}
