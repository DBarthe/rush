use shell::CommandErr;
use shell::Shell;

pub static NAME: &'static str = "eval";

pub fn builtin(args: &[~str]) -> Result<bool, CommandErr>
{
    let command = args[0].to_owned();

    let shell = Shell::new();

    shell.exec_line(command)
}
