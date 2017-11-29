use std::env;
use std::ascii::AsciiExt;
pub struct CommandStore {
    pub name: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub pipe_in: bool,
    pub pipe_out: bool,
}
impl CommandStore {
    fn new() -> CommandStore {
        CommandStore {
            name: String::new(),
            args: Vec::new(),
            stdin: None,
            stdout: None,
            stderr: None,
            pipe_in: false,
            pipe_out: false,
        }
    }
    fn add_name(&mut self, s: &str) {
        self.name = s.to_owned();
    }
    fn add_arg(&mut self, s: &str) {
        self.args.push(s.to_owned())
    }
    fn add_stdin(&mut self, s: &str) {
        self.stdin = Some(s.to_owned());
    }
    fn add_stdout(&mut self, s: &str) {
        self.stdout = Some(s.to_owned());
    }
    fn add_stderr(&mut self, s: &str) {
        self.stderr = Some(s.to_owned());
    }
}
// Token内にいるかどうかはcurrent_tokenで判断
enum ParseStatus {
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
    pub fn new(cmd: &String) -> Parser {
        let mut ps = Parser {
            parsed_cmd: Vec::new(),
            parse_status: ParseStatus::WaitCommand,
            current_token: String::new(),
            current_cmd: CommandStore::new(),
            parse_result: ParseResult::CmdOk,
        };
        for c in cmd.chars() {
            ps.read1(c);
        }
        ps
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
            '<' => ParseStatus::WaitInFile,
            '>' => ParseStatus::WaitOutFile,
            '^' => ParseStatus::WaitErrFile,
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
        self.parsed_cmd.push(self.current_cmd);
        self.current_cmd = CommandStore::new();
    }
}
