mod utils;

use crate::utils::cli_interface::{read_args, exec_args};


fn main() {
    let args = read_args();
    let res = exec_args(args);

    println!("{}", res);
}



