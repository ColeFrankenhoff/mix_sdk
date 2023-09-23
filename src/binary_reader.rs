use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


//Struct to wrap a mix word
#[derive(Debug, PartialEq, Eq)]
pub struct Word{
    is_positive: bool,
    byte_0: u8,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    byte_4: u8,
}

//Result<[Word; 4000], &'static str>

pub fn load_byte_vector(file_path: &str) -> Result<Vec<Word>,Box<dyn Error>>{
    
    let mut buffer = Vec::<u8>::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    let memory: Vec<Word> = byte_vector_to_word_array(&buffer)?;
    Ok(memory)
}


//Convert a vector of size u8 to an array of mix words representing the full
//Contents of the vm's memory
fn byte_vector_to_word_array(v: &Vec<u8>) -> Result<Vec<Word>, Box<dyn Error>>{
    let mut iter = v.chunks(6);
    let chunk: &[u8] = iter.next().unwrap();
    if chunk != &[109, 105, 120, 101, 120, 101] {
        return Err(Box::<dyn Error>::from("File doesn't contain correct header"));
    }

    let mut memory: Vec<Word> = Vec::new();
    let eof: &[u8] = &[255, 255, 255, 255, 255, 255];
    for word in iter{
        if word.len() != 6{
            return Err(Box::<dyn Error>::from("File doesn't contain lengths of multiple of six"));
        }
        if word == eof{
            continue;
        }
        let w = slice_to_word(word);
        memory.push(w);
    }
    Ok(memory)
}


fn ensure_valid_vector(v: &Vec<u8>) -> bool{
    true
}
fn slice_to_word(slice: &[u8]) -> Word{
    if slice.len() != 6{
        panic!("Incorrectly sized word slice passed");
    }
    let sign = slice[0];
    if !(sign == 0 || sign == 1){
        panic!("Sign byte read as incorrect value");
    }
    if slice.iter().max().unwrap() > &100{
        panic!("File contains too big of a value");
    }
    let b0 = slice[1];
    let b1 = slice[2];
    let b2 = slice[3];
    let b3 = slice[4];
    let b4 = slice[5];

    let sign: bool = sign != 0;
    Word{is_positive: sign, byte_0: b0, byte_1: b1, byte_2: b2, byte_3: b3, byte_4: b4}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_vector(){
        use std::fs;
        let valid_file_content: &[u8] = &[109, 105, 120, 101, 120, 101, 0, 1, 2, 3, 4, 5, 255, 255, 255, 255, 255, 255];
        let test_file_path = "valid_test.mixexe";
        fs::write(test_file_path, valid_file_content).expect("Unable to write test file");
        let expected_result = Word{
            is_positive: true,
            byte_0: 1,
            byte_1: 1,
            byte_2: 1,
            byte_3: 1,
            byte_4: 1,
        };
        let v = load_byte_vector(test_file_path).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], expected_result) 
        


    }
    fn test_read_to_byte_vector() {
        use std::fs;
        println!("Test of binary file called");
        // Create a temporary file for testing
        let test_file_content: &[u8] = &[1, 2, 3, 4, 5, 6];
        let test_file_path = "test_file.mixexe";
        fs::write(test_file_path, test_file_content).expect("Unable to write test file");

        // Call the function and check the result
        let result = load_byte_vector(test_file_path);
        assert!(result.is_ok());

        let buffer = result.unwrap();
        assert_eq!(buffer, test_file_content);

        // Clean up the temporary file
        fs::remove_file(test_file_path).expect("Unable to remove test file");
    }
    
    #[test]
    fn test_byte_to_words(){
        let v: Vec<u8> = vec![1,2,3,4,5, 6];
        let array = byte_vector_to_word_array(&v);

    }
    
    #[test]
    fn test_slice_to_word(){
        let v: &[u8] = &[0,1,2,3,4,5];
        let w = slice_to_word(v);

        dbg!(w);
        

        panic!("Another half assed test")
    }

    

    // More unit tests for other functions...
}
