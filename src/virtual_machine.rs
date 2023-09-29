#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Word{
    pub is_negative: bool,
    pub byte_1: u8,
    pub byte_2: u8,
    pub byte_3: u8,
    pub byte_4: u8,
    pub byte_5: u8,
}

//A MIX Word, with a sign bit and five one-indexed bytes
impl Word{
    pub fn zero()->Word{
        Word {is_negative: true, byte_1: 0, byte_2: 0, byte_3: 0, byte_4: 0, byte_5: 0}
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TwoByteWord{
    pub is_negative: bool,
    pub byte_1: u8,
    pub byte_2: u8,
}

impl TwoByteWord{

    pub fn zero()->TwoByteWord{
        TwoByteWord {is_negative: false, byte_1: 0, byte_2: 0}
    }
    
    //Given the size of a byte, return the value stored in a TwoByteWord
    pub fn get_value(&self, byte_size: i32)->i32{
        let x = self.byte_1 as i32;
        let y = self.byte_2 as i32;
        match self.is_negative {
        false=>x * byte_size + y,
        _=>x * -byte_size + -y
        }
    }
    
    ///Given the size of a byte and an integer store the specified integer value in self
    pub fn store_value(&mut self, value_to_store: i32, byte_size: i32){
        self.is_negative = !(value_to_store > 0);
        if byte_size > 100 || byte_size < 64{
            panic!("Byte size out of range");
        }
        if value_to_store.abs() > (byte_size * byte_size - 1){
            panic!("Value too large to store");
        }

        self.byte_1 = (value_to_store / byte_size).abs() as u8;
        self.byte_2 = (value_to_store % byte_size).abs() as u8;
    }
    
    ///Add the value of another TwoByteWord to the value of this one, accounting for sign
    ///To_add: Another TwoByteWord, such as an index register 
    ///Word_size: The amount of values a word can store, which is the 
    pub fn add(&mut self, to_add: &TwoByteWord, byte_size: i32){
        let v = self.get_value(byte_size);
        let v1 = to_add.get_value(byte_size);
        self.store_value(v + v1, byte_size);
    }
}

pub enum Comparison{
    Equal,
    Less,
    Greater
}

#[allow(non_snake_case)]
pub struct VirtualMachine{
    memory: Vec<Word>,
    pub byte_size: i32,
    pub rsp: u32,
    pub rA: Word,
    pub rX: Word,
    pub rJ: TwoByteWord,
    pub rI1: TwoByteWord,
    pub rI2: TwoByteWord,
    pub rI3: TwoByteWord,
    pub rI4: TwoByteWord,
    pub rI5: TwoByteWord,
    pub rI6: TwoByteWord,
    pub overflow_toggle: bool,
    pub comparison_toggle: Comparison
}

#[allow(non_snake_case)]
impl VirtualMachine{

    //Construct a new instance of a virtual machine memory with default values
    //for registers, values for the memory cells, a byte size, and the rsp
    ///memory: Vector of words, rsp: the first instruction to run, byte_size: size of bytes
    pub fn new(memory: Vec<Word>, rsp: u32, byte_size: i32) -> Self{
        let overflow_toggle = false;
        let comparison_toggle = Comparison::Equal;
        let rA = Word::zero();
        let rX = Word::zero();
        let rJ = TwoByteWord::zero();
        let rI1 = TwoByteWord::zero();
        let rI2 = TwoByteWord::zero();
        let rI3 = TwoByteWord::zero();
        let rI4 = TwoByteWord::zero();
        let rI5 = TwoByteWord::zero();
        let rI6 = TwoByteWord::zero();

        if 64 > byte_size || byte_size > 100{
            panic!("Invalid byte size provided to VirtualMachine constructor");
        }
        VirtualMachine{
            memory,
            rsp,
            byte_size,
            rA,
            rX,
            rJ,
            rI1,
            rI2,
            rI3,
            rI4,
            rI5,
            rI6,
            overflow_toggle,
            comparison_toggle
        }
    }
    pub fn get_word(&self, address: u32) -> Result<Word, &'static str>{
        if self.memory.len() as u32 - 1 < address{
            return Err("Index out of range");
        }

        if self.memory.len() > 4000{
            return Err("Memory stores way too many possible values")
        }
        let v = self.memory[address as usize];
        Ok(v)
    }

    pub fn set_word(&mut self, address: u32, updated_value: Word) -> Result<(), &'static str>{
        if address > self.memory.len() as u32 - 1{
            return Err("Index out of range");
        }

        self.memory[address as usize] = updated_value;
        Ok(())
    }
}


//ChatGPT Generated tests which are bad but not useless
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_zero() {
        let word = Word::zero();
        assert_eq!(
            word,
            Word {
                is_negative: true,
                byte_1: 0,
                byte_2: 0,
                byte_3: 0,
                byte_4: 0,
                byte_5: 0
            }
        );
    }

    #[test]
    fn test_two_byte_word_zero() {
        let two_byte_word = TwoByteWord::zero();
        assert_eq!(
            two_byte_word,
            TwoByteWord {
                is_negative: false,
                byte_1: 0,
                byte_2: 0
            }
        );
    }


    #[test]
    fn test_get_word() {
        let memory = vec![Word::zero(), Word::zero(), Word::zero()];
        let vm = VirtualMachine::new(memory, 0, 64);
        match vm.get_word(1) {
            Ok(word) => {
                assert_eq!(word, Word::zero());
            }
            Err(_) => {
                // Error case, handle as needed
            }
        }
    }

    // Example test for set_word function
    #[test]
    fn test_set_word() {
        let mut memory = vec![Word::zero(), Word::zero(), Word::zero()];
        let mut vm = VirtualMachine::new(memory, 3, 100);
        let new_word = Word {
            is_negative: false,
            byte_1: 1,
            byte_2: 2,
            byte_3: 3,
            byte_4: 4,
            byte_5: 5,
        };

        // Set a word at index 1
        match vm.set_word(1, new_word.clone()) {
            Ok(_) => {
                // Verify the word was set
                assert_eq!(vm.get_word(1), Ok(new_word));
            }
            Err(_) => {
                // Error case, handle as needed
            }
        }
    }
    #[test]
    fn test_get_two_byte_value(){
        let mut new_two_byte_word = TwoByteWord{
            is_negative: false,
            byte_1: 1,
            byte_2: 2,
        };
        assert_eq!(new_two_byte_word.get_value(64), 66);
        new_two_byte_word.is_negative = true;
        assert_eq!(new_two_byte_word.get_value(64), -66);
    }
    
    #[test]
    fn test_store_two_byte_value(){
        let mut two_byte = TwoByteWord::zero();

        for val in -200..200{
            for byte in 64..101{
                two_byte.store_value(val, byte);
                assert_eq!(two_byte.get_value(byte), val);
            }
        }
    }


    #[test]
    fn test_addition(){
        let mut w1 = TwoByteWord::zero();
        let mut w2 = TwoByteWord::zero();

        for value_one in 50..300{
            for byte_size in 64..101{
                let value_two = 50 - 2 * value_one;
                w1.store_value(value_one, byte_size);
                w2.store_value(value_two, byte_size);
                w1.add(&w2, byte_size);
                assert_eq!(w1.get_value(byte_size), value_one + value_two);
            }
        }
    }
}
