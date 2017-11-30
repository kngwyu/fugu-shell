use common::LOGGER;
use exec::CommandStore;
// Token内にいるかどうかはcurrent_tokenで判断
#[derive(Clone, Copy, Debug)]
pub enum ParseStatus {
    WaitCommand,
    WaitArgs,
    WaitInFile,
    WaitOutFile,
    WaitErrFile,
}
enum ParseResult {
    CmdOk,
}
pub struct Parser {
    parsed_cmd: Vec<CommandStore>,
    parse_status: ParseStatus,
    current_token: String,
    current_cmd: CommandStore,
    parse_result: ParseResult,
}
impl Parser {
    pub fn new() -> Parser {
        Parser {
            parsed_cmd: Vec::new(),
            parse_status: ParseStatus::WaitCommand,
            current_token: String::new(),
            current_cmd: CommandStore::new(),
            parse_result: ParseResult::CmdOk,
        }
    }
    pub fn add_str(&mut self, s: &str) {
        for c in s.chars() {
            self.read1(c);
        }
    }
    pub fn read1(&mut self, ch: char) {
        self.parse_status = match ch {
            ' ' => self.add_token(),
            ';' => {
                self.add_cmd();
                ParseStatus::WaitCommand
            }
            '|' => {
                self.current_cmd.pipe_out = true;
                self.add_cmd();
                self.current_cmd.pipe_in = true;
                ParseStatus::WaitCommand
            }
            '&' => {
                self.current_cmd.wait = false;
                self.add_cmd();
                ParseStatus::WaitCommand
            }
            '<' => ParseStatus::WaitInFile,
            '>' => ParseStatus::WaitOutFile,
            '^' => {
                if self.current_token.is_empty() {
                    ParseStatus::WaitErrFile
                } else {
                    self.current_token.push(ch);
                    self.parse_status
                }
            }
            _ => {
                self.current_token.push(ch);
                self.parse_status
            }
        };
    }
    fn add_token(&mut self) -> ParseStatus {
        if self.current_token.len() == 0 {
            return self.parse_status;
        }
        match self.parse_status {
            ParseStatus::WaitCommand => {
                self.current_cmd.add_name(&self.current_token);
                self.current_token.clear();
                ParseStatus::WaitArgs
            }
            ParseStatus::WaitArgs => {
                self.current_cmd.add_arg(&self.current_token);
                self.current_token.clear();
                ParseStatus::WaitArgs
            }
            ParseStatus::WaitInFile => {
                self.current_cmd.add_stdin(&self.current_token);
                ParseStatus::WaitArgs
            }
            ParseStatus::WaitOutFile => {
                self.current_cmd.add_stdout(&self.current_token);
                ParseStatus::WaitArgs
            }
            ParseStatus::WaitErrFile => {
                self.current_cmd.add_stdin(&self.current_token);
                ParseStatus::WaitArgs
            }
        }
    }
    fn add_cmd(&mut self) {
        self.parsed_cmd.push(self.current_cmd.clone());
        self.current_cmd = CommandStore::new();
    }
}
