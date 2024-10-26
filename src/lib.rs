use std::{env, io::Read, path::PathBuf};
use clap::{arg, command, ArgMatches, Command};
use std::fs::File;

//Read in all arguments
pub fn read_args() -> ArgMatches {

    //Read the command line arguments
    command!()
    .arg(arg!([Search] "Text to be searched for"))
    .arg(arg!([File] "File to be searched"))
    .subcommand(Command::new("test").about("Prints out a test file"))
    .get_matches()
}

//TODO---Execute Args---//
pub fn exec_args(_args: ArgMatches){
    todo!()
}

//---ReadFile---//
pub fn read_file(filepath: PathBuf) -> String {

    //TODO Impliment error handling for opening file
    let mut file = File::open(filepath).expect("Could not open file in read_file");

    let mut content: String = String::new();

    file.read_to_string(&mut content).expect("Something went wrong with reading the test file");

    content
}


//TODO---Search Function---//

//TODO---Testing---//

//Open file test and print out text

fn print_test_file() -> String{
    let mut file = File::open("Test.txt").expect("Something went wrong with opening the test file");

    let mut content: String = String::new();

    file.read_to_string(&mut content).expect("Something went wrong with reading the test file");

    print!("{}",content);

    content
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    #[test]
    fn test_file_reader(){
        fs::write("tests/t1.txt", "Lorem\nIpsum\n\ndolor sit amet").expect("Writing to test file failed");
        let mut file = File::open("tests/t1.txt").expect("Failed to read test file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read test file content");

        assert_eq!(contents, read_file(PathBuf::from("tests/t1.txt")));
    }
}