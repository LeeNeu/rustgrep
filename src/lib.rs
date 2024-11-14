use std::{env, error::Error};
use clap::{arg, command, ArgMatches, Command};
use std::fs;
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
    // TODO instead of unwrap the Error should be mapped when returned
    let haystack= read_file(filepath.clone()).unwrap();
    return search(search_string, haystack.as_str());
}

//---ReadFile---//
fn read_file(filepath: String) -> Result<String, Box<dyn Error>> {
    let content: String = fs::read_to_string(filepath)?;
    Ok(content)
}


//TODO---Search Function---//

fn search(search_string: &str, haystack: &str) -> String{
    // Regex looks for the searched string in the given file and returns the line in which it is found
    let regex = Regex::new(format!("{}{}{}",r"\n.*", search_string,r".*[\n\r]").as_str()).unwrap();
    let mat = regex.find(haystack).unwrap();
    mat.as_str().to_string()
}

//---Testing---//

//Open file test and print out text
fn print_test_file(){
    println!("{}",read_file("testfiles/Test.txt".to_string()).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    #[test]
    fn test_file_reader(){
        fs::write("testfiles/t1.txt", "Lorem\nIpsum\n\ndolor sit amet").expect("Writing to test file failed");
        let contents = fs::read_to_string("testfiles/t1.txt").expect("Failed to read test file");
        assert_eq!(contents, read_file("testfiles/t1.txt".to_string()).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_file_reader_err(){
        read_file("Not_A_File".to_string()).unwrap();
    }

    #[test]
    fn test_search(){
        let content = read_file("testfiles/Test.txt".to_string()).unwrap();
        let ans = search("locked",&content);

        assert_eq!(ans, "\nam I to be locked in this\n".to_string());
    }
}