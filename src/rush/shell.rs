 
use collections::HashMap;
use std::io::{print, println};
use std::io;

use builtins;
use builtins::exit;
use cmd_reader::CmdReader;
use parser::Parser;

pub type BuiltinFn = fn (args: &[~str]) -> Result<bool, CommandErr>;

pub struct Shell {
    prompt: ~str,

    parser: Box<Parser>,

    builtins: Box<HashMap<~str, BuiltinFn>>,
}

pub enum CommandErr {
    CommandNotFound(~str),
}

impl Shell {

    pub fn new() -> Shell {
        Shell {
            prompt: "$ ".to_owned(),
            builtins: builtins::create_builtins(),
            parser: box Parser::new(),
        }
    }

    pub fn run(&self, reader: &mut CmdReader, show_prompt: bool) {
        if show_prompt {
            println("Rush started! Press Ctrl+D or type 'quit' to quit.");
        }

        loop {

            if show_prompt {
                print(self.prompt);
            }

            let line = match reader.read_line() {
                Ok(line) => line,
                Err(e) => {
                    if e.kind == io::EndOfFile {
                        // avoid warning temporary
                        match exit::builtin([]) {
                            _ => {}
                        };
                    }
                    else {
                        println!("rush: IoError: {}", e);
                    }
                    break;
               },
            };

            match self.exec_line(line) {
                Ok(stop) if stop => { break; }
                Ok(_) => {},
                Err(e) => match e {
                    CommandNotFound(command) => println!("Command not found: {:s}", command),
                },
            }
        }
    }

    pub fn exec_line(&self, line: ~str) -> Result<bool, CommandErr> {
        self.exec(line)
    }

    // command will be replaced by a Command type (pipe, redirs, etc...)
    fn exec(&self, command: ~str) -> Result<bool, CommandErr> {
        let words: Vec<~str> = command.words().map(|s| s.to_owned()).collect();

        if words.len() == 0 {
            return Ok(false);
        }

        let program = words.get(0).to_owned();
        let args = words.slice_from(1);

        match self.builtins.find(&program) {
            Some(handler) => (*handler)(args),
            None => self.exec_process(program, args),
        }
    }

    fn exec_process(&self, program: ~str, args: &[~str]) -> Result<bool, CommandErr> {
        use std::io::process;
        use std::io::process::Command;

        println!("executing program '{:s}'", program);

        let mut command = Command::new(program.to_owned());
        command.args(args)
                .stdin(process::InheritFd(0))
                .stdout(process::InheritFd(1))
                .stderr(process::InheritFd(2));

        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(_) => {
                return Err(CommandNotFound(program.to_owned()));
            },
        };

        child.wait().unwrap();
        println!("done");
        Ok(false)
    }

}
