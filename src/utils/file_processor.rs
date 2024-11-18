use std::{fs,error::Error};
use regex::Regex;
use search_parameters::SearchParameters;

//---ReadFile---//
// Read file into String and return io::Read error if not possible
pub fn read_file(filepath: String) -> Result<String, Box<dyn Error>> {
    let content: String = fs::read_to_string(filepath)?;
    Ok(content)
}


//---Search Function---//
//TODO make search function return result
pub fn search(params: SearchParameters) -> String{

    // Regex looks for the searched string in the given file and returns the line in which it is found
    let regex = Regex::new(format!("{}{}{}",r"\n.*", params.search_string,r".*[\n\r]").as_str()).unwrap();
    let mat = regex.find(&params.haystack).unwrap();

    //TODO find a better way to output it instead of conversioning back and forth
    mat.as_str().to_string()
}

//TODO Implement Submodule for Search Parameters struct
pub mod search_parameters{
    // Contains search parameters to pass to search function
    pub struct SearchParameters {
        pub search_string: String,
        pub haystack: String,
    }
}

#[cfg(test)]
mod test {
    use super::*;
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

        let search_params = SearchParameters{
            search_string: "locked".to_string(),
            haystack: content,
        };

        let ans = search(search_params);

        assert_eq!(ans, "\nam I to be locked in this\n".to_string());
    }
}
