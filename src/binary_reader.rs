use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use crate::virtual_machine::Word;


//Struct to wrap a mix word
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
    Word{is_negative: sign, byte_0: b0, byte_1: b1, byte_2: b2, byte_3: b3, byte_4: b4}
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
            is_negative: false,
            byte_0: 1,
            byte_1: 2,
            byte_2: 3,
            byte_3: 4,
            byte_4: 5,
        };
        let v = load_byte_vector(test_file_path).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], expected_result) 
        

    }
    
}
