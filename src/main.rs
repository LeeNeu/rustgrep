mod utils;

use crate::utils::cli_interface::exec_args;

fn main() {
    //TODO Implement error handling in main
    let res = exec_args().unwrap();

    println!("{}", res);
}
