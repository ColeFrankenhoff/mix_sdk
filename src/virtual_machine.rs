#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Word{
    pub is_negative: bool,
    pub byte_0: u8,
    pub byte_1: u8,
    pub byte_2: u8,
    pub byte_3: u8,
    pub byte_4: u8,
}
impl Word{
    pub fn zero()->Word{
        Word {is_negative: true, byte_0: 0, byte_1: 0, byte_2: 0, byte_3: 0, byte_4: 0}
    }
}
#[derive(Debug, PartialEq, Eq, Default)]
pub struct TwoByteWord{
    is_negative: bool,
    byte_0: u8,
    byte_1: u8,
}

impl TwoByteWord{
    pub fn zero()->TwoByteWord{
        TwoByteWord {is_negative: false, byte_0: 0, byte_1: 0}
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
    pub byte_size: u32,
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
    fn new(memory: Vec<Word>, rsp: u32, byte_size: u32) -> Self{
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
                byte_0: 0,
                byte_1: 0,
                byte_2: 0,
                byte_3: 0,
                byte_4: 0
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
                byte_0: 0,
                byte_1: 0
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
            byte_0: 1,
            byte_1: 2,
            byte_2: 3,
            byte_3: 4,
            byte_4: 5,
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

}
