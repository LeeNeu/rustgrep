use std::env;
use clap::{arg, command, Command};

fn main() {
    let args = command!()
        .arg(arg!([Search] "Text to be searched for"))
        .arg(arg!([File] "File to be searched"))
        .subcommand(Command::new("test").about("Command for testing"))
        .get_matches();



    print!("{:?}", args);
}

