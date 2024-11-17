use crate::utils::file_processor::read_file;

//Open file "testfiles/Test.txt" and print out text
pub fn print_test_file(){
    println!("{}",read_file("testfiles/Test.txt".to_string()).unwrap());
}