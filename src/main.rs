mod utils;

use crate::utils::cli_interface::{exec_args, read_args};

fn main() {
    let args = read_args();

    //TODO Implement error handling in main
    let res = exec_args(args).unwrap();

    println!("{}", res);
}
