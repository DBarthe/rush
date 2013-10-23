use std::os;
use rush::shared::*;

pub static NAME: &'static str = "exit";

pub fn builtin_exit(args: &[~str]) -> Result<bool, CommandErr>
{
    println("Goodbye. :)");

    let mut status = 0;
    if args.len() == 1
    {
        status = match from_str(args[0]) {
            Some(code) => code,
            None => 1,
        };
    }

    os::set_exit_status(status);

    Ok(true)
}