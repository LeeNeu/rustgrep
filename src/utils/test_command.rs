use crate::utils::file_processor::read_file;

//TODO implement that this code only gets compiled when it is a test build
const TESTPATH: &str = "testfiles/Test.txt";

//Open file "testfiles/Test.txt" and print out text
pub fn print_test_file() {
    println!("{}", read_file(TESTPATH.to_string()).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    #[test]
    fn check_test_filepath() {
        File::open(TESTPATH).unwrap();
    }
}



