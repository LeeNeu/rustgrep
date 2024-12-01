use crate::utils::file_processor::{read_file, search, search_parameters::SearchParameters};
use crate::utils::test_command::print_test_file;
use clap::{arg, command, ArgMatches, Command};
use std::error::Error;

//Read in all arguments
// TODO Implement Piping support
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
pub fn exec_args(args: ArgMatches) -> Result<String, Box<dyn Error>> {
    // TODO Create an Error type in case test is called
    if args.subcommand_matches("test").is_some() {
        print_test_file();
        return Ok("".to_string());
    }

    // Get arguments
    let search_string = args.get_one::<String>("string").unwrap();
    let filepath = args.get_one::<String>("filepath").unwrap();

    // Execute file search
    let haystack = read_file(filepath.clone())?;

    //Construct search paramters to pass to search function
    let search_params = SearchParameters {
        search_string: search_string.clone(),
        haystack,
    };

    // TODO create custom error in case not Regex could be matched
    Ok(search(search_params).unwrap())
}

//TODO Implement Tests for interface
#[cfg(test)]
mod test {
    // use super::*;
}
