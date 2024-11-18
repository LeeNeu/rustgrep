mod utils;

use crate::utils::cli_interface::{read_args, exec_args};


fn main() {
    let args = read_args();

    //TODO Implement error handling in main
    let res = exec_args(args).unwrap();

    println!("{}", res);
}



