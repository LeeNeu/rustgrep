use crate::prelude::error::SearchParameterError;
use crate::utils::file_processor::{read_file, search, search_parameters::SearchParameters};
use clap::Parser;
use std::{error::Error, io, io::IsTerminal};

#[cfg(feature = "test_command")]
use {crate::utils::test_command::print_test_file, clap::Subcommand};

//Derive Argument Parser
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    ///String to be searched for
    string: Option<String>,

    ///File to be searched
    filepath: Option<String>,

    #[cfg(feature = "test_command")]
    #[command(subcommand)]
    command: Option<Commands>,
}

#[cfg(feature = "test_command")]
#[derive(Subcommand)]
enum Commands {
    ///Prints out test file
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

    //If Filepath can be retrieved from arguments read into haystack with read_file,
    //If not check pipe input and if available read it into haystack
    let haystack = match cli.filepath {
        Some(path) => read_file(path.clone())?,
        None => {
            //Check piped input error
            get_piped()?
        }
    };

    //Construct search paramters to pass to search function
    let search_params = SearchParameters {
        search_string: cli
            .string
            .ok_or(SearchParameterError::Empty("STRING".to_string()))?,
        haystack,
    };

    // Search for string in haystack
    search(search_params)
}

// Extracts piped argument from stdin
fn get_piped() -> Result<String, SearchParameterError> {
    // In case there is no piped input return error
    if io::stdin().is_terminal() {
        return Err(SearchParameterError::Empty("FILEPATH".to_string()));
    }
    //TODO cap max byte size to be read
    let input =
        io::read_to_string(io::stdin()).map_err(|_| SearchParameterError::FalsePipeInput)?;
    Ok(input)
}

//TODO Implement Tests for interface
#[cfg(test)]
mod test {
    // use super::*;
}
