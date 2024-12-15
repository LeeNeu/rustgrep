use crate::utils::file_processor::{read_file, search, search_parameters::SearchParameters};
//use clap::{arg, ArgMatches, Command};
use clap::Parser;
use std::error::Error;

#[cfg(feature = "test_command")]
use  {crate::utils::test_command::print_test_file, clap::Subcommand};


//Derive Argument Parser
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    //String to be searched for
    string: Option<String>,

    //File to be searched
    filepath: Option<String>,

    #[cfg(feature = "test_command")]
    #[command(subcommand)]
    command: Option<Commands>,
    
}

#[cfg(feature = "test_command")]
#[derive(Subcommand)]
enum Commands {
    //Prints out test file
    Test,
}

// TODO implement piping support

//---Execute Args---//
// Handles all args provided from the command line and executes the matching functions
pub fn exec_args() -> Result<String, Box<dyn Error>> {

    //Parse Arguments
    let cli = Cli::parse();

    #[cfg(feature = "test_command")]
    match &cli.command {
        Some(Commands::Test) => {
            print_test_file();
            return Ok("".to_string());
        }
        None => {}
    }

    // Get arguments
    // TODO let them return the None value as a result type
    let search_string = cli.string.unwrap();
    let filepath = cli.filepath.unwrap();

    // Execute file search
    let haystack = read_file(filepath.clone())?;

    //Construct search paramters to pass to search function
    let search_params = SearchParameters {
        search_string: search_string.clone(),
        haystack,
    };

    // TODO create custom error in case no Regex could be matched
    Ok(search(search_params).unwrap())
}

//TODO Implement Tests for interface
#[cfg(test)]
mod test {
    // use super::*;
}
