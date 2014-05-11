#![crate_id = "rush#0.1.0"]
// I don't know why this attribute standing for ...
//#[uuid = "636e5f0c-2815-11e3-989c-6bd39c248c79"]
#![crate_type = "bin"]
#![desc = "A shell written in Rust"]
#![license = "GPLv3"]

#![feature(globs)]

extern crate getopts;
extern crate collections;

use std::os;
use getopts::getopts;
use std::io::{print, println};

pub mod builtins;
pub mod shell;
pub mod cmd_reader;



fn start(args: Vec<~str>)
{
    let program = args.get(0).clone();
    let opts = ~[
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];
    let matches = match getopts::getopts(args.tail(), opts)
    {
        Ok(m) => m,
        Err(f) => fail!(f.to_err_msg()),
    };
    if matches.opt_present("h") || matches.opt_present("help")
    {
        println("rush 0.1.0");
        println("");
        println("Usage:");
        println!("  {:s} [file]", program);
        println("");
        print(getopts::usage("Rush Shell", opts));
        return;
    }
    if matches.opt_present("V") || matches.opt_present("version")
    {
        println("rush 0.1.0");
        return;
    }

    let mut show_prompt = true;
    let mut reader = cmd_reader::CmdReader::new();

    if !matches.free.is_empty() {
        match reader.set_to_file(matches.free.get(0).as_slice()) {
            Err(e) => {
                println!("{}: {}", matches.free.get(0), e.desc);
                return ;
            },
            _ => {}, 
        };
        show_prompt = false;
    }

    let shell = shell::Shell::new();
    shell.run(&mut reader, show_prompt);
}

fn main()
{
    let args = os::args();

    start(args);
}
