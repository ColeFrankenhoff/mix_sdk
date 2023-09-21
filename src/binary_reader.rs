use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


//Struct to wrap a mix word
pub struct Word{
    sign: bool,
    byte_0: u8,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    byte_4: u8,
}


//Result<[Word; 4000], &'static str>

pub fn mixexe_to_byte_vector(file_path: &str) -> Result<Vec<u8>,Box<dyn Error>>{
    
    let mut buffer = Vec::<u8>::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    
    for chunk in buffer.chunks(6){
        println!("{:?}", chunk);
    }
    Ok(buffer)
}




//Stub 
fn ensure_valid_vector(v: &Vec<u8>) -> bool{
    true
}

//Convert a vector of size u8 to an array of mix words representing the full
//Contents of memory
fn byte_vector_to_word_array(v: &Vec<u8>){
    let mut iter = v.chunks(6);
    let chunk = iter.next();
    println!("{:?}", chunk);
    let chunk = iter.next();
    println!("{:?}", chunk);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_to_byte_vector() {
        use std::fs;
        println!("Test of binary file called");
        // Create a temporary file for testing
        let test_file_content: &[u8] = &[1, 2, 3, 4, 5, 6];
        let test_file_path = "test_file.mixexe";
        fs::write(test_file_path, test_file_content).expect("Unable to write test file");

        // Call the function and check the result
        let result = mixexe_to_byte_vector(test_file_path);
        assert!(result.is_ok());

        let buffer = result.unwrap();
        assert_eq!(buffer, test_file_content);

        // Clean up the temporary file
        fs::remove_file(test_file_path).expect("Unable to remove test file");
    }
    
    #[test]
    fn test_byte_to_words(){
        let v: Vec<u8> = vec![1,2,3,4,5, 6];
        byte_vector_to_word_array(&v);
        panic!("This is a half-assed test")
    }
    

    // More unit tests for other functions...
}
