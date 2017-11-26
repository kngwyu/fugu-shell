use std::env;
struct Parser {}
impl Parser {}
type TokenList = Vec<String>;
pub enum ParseStatus {

}
pub fn parse_cmd(command: &String) -> Vec<CommandStore> {
    let mut res = Vec::new();
    // 直前のRedirect Operation
    enum RedirectOpe {
        None,
        Stdin,
        Stdout,
    }
    let mut cur_state = RedirectOpe::None;
    let mut cur_command: Option<CommandStore> = None;
    for token in command.split_whitespace() {
        match cur_state {
            RedirectOpe::None => {
                match token {
                    "<" => cur_state = RedirectOpe::Stdin,
                    ">" => cur_state = RedirectOpe::Stdout,
                    "|" => {
                        if let Some(cmd) = cur_command {
                            res.push(cmd);
                            cur_command = None;
                        }
                    }
                    _ => {
                        if let Some(mut cmd) = cur_command {
                            cmd.add_arg(token);
                            cur_command = Some(cmd);
                        } else {
                            cur_command = Some(CommandStore::new(token));
                        }
                    }
                }
            }
            RedirectOpe::Stdin => {
                if let Some(mut cmd) = cur_command {
                    cmd.add_stdin(token);
                    cur_command = Some(cmd);
                } else {
                    println!("Fugu: invalid use of <");
                }
            }
            RedirectOpe::Stdout => {
                if let Some(mut cmd) = cur_command {
                    cmd.add_stdout(token);
                    cur_command = Some(cmd);
                } else {
                    println!("Fugu: invalid use of >");
                }
            }
        }
    }
    if let Some(cmd) = cur_command {
        res.push(cmd);
    }
    res
}
