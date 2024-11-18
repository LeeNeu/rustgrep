use std::error::Error;

use clap::{arg, command, ArgMatches, Command};
use crate::utils::test_command::print_test_file;
use crate::utils::file_processor::{search, read_file};

//Read in all arguments
pub fn read_args() -> ArgMatches {
    //Read the command line arguments
    command!()
    .arg(arg!([string] "String to be searched for"))
    .arg(arg!([filepath] "File to be searched"))
    .subcommand(Command::new("test").about("Prints out a test file"))
    .get_matches()
}

//---Execute Args---//
// Handles all args provided from the command line and executes the matching functions
pub fn exec_args(args: ArgMatches)-> Result<String,Box<dyn Error>>{

    // TODO make this function an option type in case test is called and no String is returned
    if args.subcommand_matches("test").is_some() {
        print_test_file();
        return Ok("".to_string());
    }

    // Get arguments
    let search_string = args.get_one::<String>("string").unwrap();
    let filepath = args.get_one::<String>("filepath").unwrap();

    // Execute file search
    // TODO instead of unwrap the Error should be mapped when returned
    let haystack= read_file(filepath.clone())?;
    Ok(search(search_string, &haystack))
}

//TODO Implement Tests for interface
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec_args(){

    }
}