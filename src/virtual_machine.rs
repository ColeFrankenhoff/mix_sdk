use std::process;

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
        Word {is_negative: false, byte_1: 0, byte_2: 0, byte_3: 0, byte_4: 0, byte_5: 0}
    }

    pub fn get_value(&self, byte_size: i32)->i32{
        if byte_size > 100 || byte_size < 64 {
            panic!("Invalid bytesize")
        }
        

        let b1 = self.byte_1 as i32;
        let b2 = self.byte_2 as i32;
        let b3 = self.byte_3 as i32;
        let b4 = self.byte_4 as i32;
        let b5 = self.byte_5 as i32;
        let magnitude = b5 + b4 * byte_size + b3 * byte_size.pow(2) + b2 * byte_size.pow(3) + b1 * byte_size.pow(4);
        match self.is_negative{
            true=>magnitude * -1,
            _=> magnitude
        }
        

    }

    //Todo: bugs involving positive and negative zero
    ///Parameters: value -i32, byte_size: i32
    pub fn store_value(&mut self, value: i32, byte_size: i32){
        if byte_size > 100 || byte_size < 64 {
            panic!("Invalid bytesize")
        }
        

        let mut val = value.abs();
        self.byte_5 = (val % byte_size) as u8;
        val /= byte_size;
        self.byte_4 = (val % byte_size) as u8;
        val /= byte_size;
        self.byte_3 = (val % byte_size) as u8;
        val /= byte_size;
        self.byte_2 = (val % byte_size) as u8;
        val /= byte_size;
        self.byte_1 = (val % byte_size) as u8;

        self.is_negative = value < 0;
        dbg!(self.is_negative);
    }
    pub fn add(&mut self, to_add: &Word, byte_size: i32){
        let x = to_add.get_value(byte_size);
        let y = x + self.get_value(byte_size);
        self.store_value(y, byte_size);
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

#[derive(Default, Debug, Clone, Copy)]
struct PartialField{
    is_negative: Option<bool>,
    byte_1: Option<u8>,
    byte_2: Option<u8>,
    byte_3: Option<u8>,
    byte_4: Option<u8>,
    byte_5: Option<u8>,
}

impl PartialField{
    fn new(original_word: &Word, field: u8) -> Self{
        let mut value: PartialField = Default::default();
        
        //The field spec for L, R is 8L + R
        let right_field = field % 8;
        let left_field = (field - right_field) / 8;

        if (8* left_field + right_field) != field{
            panic!("My math is ass");
        }
        if left_field > right_field{
            panic!("Left field specification greater than right field specification");
        }


        for v in left_field..right_field + 1{
            match v{
                0 => value.is_negative = Some(original_word.is_negative),
                1 => value.byte_1 = Some(original_word.byte_1),
                2 => value.byte_2 = Some(original_word.byte_2),
                3 => value.byte_3 = Some(original_word.byte_3),
                4 => value.byte_4 = Some(original_word.byte_4),
                5 => value.byte_5 = Some(original_word.byte_5),
                _ => panic!("Invalid field supplied")
            }
        }

        value
            
    }
    fn get_value(&self, byte_size: u32)->i32{
       let b1_val = self.byte_1;
       let b2_val = self.byte_2;
       let b3_val = self.byte_3;
       let b4_val = self.byte_4;
       let b5_val = self.byte_5;
        
       let mut mag: u32 = 0;
       let bytes = [b1_val, b2_val, b3_val, b4_val, b5_val];
        
       for byte in bytes{
           
           //This is pretty cute
           let v = match byte{
               Some(v) => v,
               _ => {continue;}
           };

           mag *= byte_size;
           mag += v as u32;
       }

        match self.is_negative.unwrap_or(false){
           true=>-(mag as i32),
           false=>mag as i32
       }
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
        Ok(v.clone())
    }

    pub fn set_word(&mut self, address: u32, updated_value: Word) -> Result<(), &'static str>{
        if address > self.memory.len() as u32 - 1{
            return Err("Index out of range");
        }

        self.memory[address as usize] = updated_value;
        Ok(())
    }

    ///Sets the word at the specified addres to the updated value
    ///Parameters: adress: u32, updated_value:i32
    pub fn set_word_value(&mut self, address: u32, updated_value: i32) -> Result<(), &'static str>{
        if self.memory.len() as u32 - 1 < address{
            return Err("Index out of range");
        }

        if self.memory.len() > 4000{
            return Err("Memory stores too many possible values")
        }

        let mut v = self.memory[address as usize];
        v.store_value(updated_value, self.byte_size);
        Ok(())
    }
    pub fn get_rA_val(&self) -> i32 {
        self.rA.get_value(self.byte_size)
    }
    pub fn get_rX_val(&self) -> i32 {
        self.rX.get_value(self.byte_size)
    }
        
    ///Return the value at address designated by the specified partial field
    pub fn load_v(&self, address: u32, partial_field: u8) -> Result<i32, &'static str>{ 

        let word = match self.get_word(address){
            Ok(word) => word,
            Err(e) => return Err(e),
        };
        
        let v = PartialField::new(&word, partial_field);
        dbg!(v);
        Ok(v.get_value(self.byte_size as u32))

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_zero() {
        let word = Word::zero();
        assert_eq!(
            word,
            Word {
                is_negative: false,
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

    #[test]
    fn test_get_word_get_value(){
        let mut word = Word::zero();
        assert_eq!(word.get_value(64), 0);

        word.byte_1 = 1;
        assert_eq!(word.get_value(64), 16777216);
    
    }

    #[test]
    fn test_store_word_value(){
        let mut word = Word::zero();

        for val in -2000..2000{
            for byte in 64..101{
                word.store_value(val, byte);
                assert_eq!(word.get_value(byte), val);
            }
        }
    }

    #[test]
    fn word_addition_test(){
        let mut w1 = Word::zero();
        let mut w2 = Word::zero();

        //Yes, this is an unnecessary and useless amount of test
        //cases and I should have the edge cases tested. But rust runs this quickly which is
        //very satisfying
        for val in -20000..20000{
            for byte in 64..101{
                let x = val;
                let y = val * val % 4;
                w1.store_value(x, byte);
                w2.store_value(y, byte);
                w1.add(&w2, byte);
                assert_eq!(w1.get_value(byte), x + y);
            }
        }
    }


    #[test]
    fn partial_fields(){
        let mut word = Word::zero();

        let partial_word = PartialField::new(&word, 5);
        assert_eq!(partial_word.get_value(64), 0);
        

        word.is_negative = true;
        word.byte_1 = 50;
        word.byte_2 = 50;
        
        let expected = -3250;
        let partial = PartialField::new(&word, 2);
        assert_eq!(expected, partial.get_value(64));
        
        word.byte_3 = 60;
        let partial_2 = PartialField::new(&word, 19);
        let expected = 3260;

        assert_eq!(expected, partial_2.get_value(64));

        word.byte_1 = 50;
        word.byte_2 = 0;
        word.is_negative = true;
        
        let partial = PartialField::new(&word, 3);

        let expect = (-3200 * 64) - 60;
        assert_eq!(expect, partial.get_value(64));
    }


    #[test]
    fn load_v(){

        let mut w1 = Word::zero();
        w1.is_negative = true;
        w1.byte_1 = 50;
        w1.byte_2 = 0;
        let memory = vec![w1, Word::zero(), Word::zero()];
        let vm = VirtualMachine::new(memory, 0, 64);
        let val = vm.load_v(0, 9).unwrap();
        assert_eq!(val, 50);


        let val = vm.load_v(0, 10).unwrap();
        assert_eq!(val, 3200);


        let val = vm.load_v(0, 3).unwrap();
        assert_eq!(val, -3200 * 64);
 
    }
}
