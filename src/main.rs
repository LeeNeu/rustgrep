mod prelude;
mod utils;

use crate::utils::cli_interface::exec_args;

fn main() {
    //TODO Implement error handling in main
    match exec_args() {
        Ok(res) => println!("{}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}
