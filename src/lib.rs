use std::{env, io::Read};
use clap::{arg, command, ArgMatches, Command};
use std::fs::File;
use regex::Regex;

//Read in all arguments
pub fn read_args() -> ArgMatches {

    //Read the command line arguments
    command!()
    .arg(arg!([string] "String to be searched for"))
    .arg(arg!([filepath] "File to be searched"))
    .subcommand(Command::new("test").about("Prints out a test file"))
    .get_matches()
}

//TODO---Execute Args---//
// Handles all args provided from the command line and executes the matching functions
pub fn exec_args(args: ArgMatches)-> String{

    // TODO make this function an option type in case test is called and no String is returned
    if args.subcommand_matches("test").is_some() {
        print_test_file();
        return "".to_string();
    }

    // Get arguments
    let search_string = args.get_one::<String>("string").unwrap();
    let filepath = args.get_one::<String>("filepath").unwrap();

    // Execute file search
    let haystack= read_file(filepath.clone());
    return search(search_string, haystack.as_str());
}

//---ReadFile---//
fn read_file(filepath: String) -> String {

    //TODO Impliment error handling for opening file
    let mut file = File::open(filepath).expect("Could not open file in read_file");

    let mut content: String = String::new();

    file.read_to_string(&mut content).expect("Something went wrong with reading the test file");

    content
}


//TODO---Search Function---//

fn search(search_string: &str, haystack: &str) -> String{
    // Regex looks for the searched string in the given file and returns the line in which it is found
    let regex = Regex::new(format!("{}{}{}",r"\n.*", search_string,r".*[\n\r]").as_str()).unwrap();
    let mat = regex.find(haystack).unwrap();
    mat.as_str().to_string()
}

//TODO---Testing---//

//Open file test and print out text
fn print_test_file(){
    println!("{}",read_file("tests/Test.txt".to_string()))
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

        assert_eq!(contents, read_file("tests/t1.txt".to_string()));
    }

    #[test]
    fn test_search(){
        let content = read_file("tests/Test.txt".to_string());
        let ans = search("locked",&content);

        assert_eq!(ans, "\nam I to be locked in this\n".to_string());
    }
}