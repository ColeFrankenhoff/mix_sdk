//!Define a single public function 'run', which takes a virtual_machine as an argument 
//!and executes the code. Stops after a reasonable amount of MIX units of time
use crate::virtual_machine::VirtualMachine;
use crate::virtual_machine::Word;
use crate::virtual_machine::TwoByteWord;



///Execute the contents of a virtual machine, starting at the value specified by rsp
///Take ownership of the vm's memory
pub fn run_vm(mut virtual_machine: VirtualMachine){
    
    let mut execution_time = 0;
    loop{
        let rsp = virtual_machine.rsp;
        let w = virtual_machine.get_word(rsp).unwrap();
        
        let res = call_instruction(&mut virtual_machine, w);

        if let InstructionResult::ExecutionTime(c) = res{
            execution_time += c;
        }
        else{
            break;
        }
    }
}


//Each MIXAL instruction will either return an execution time or a halt
enum InstructionResult{
    HLT,
    ExecutionTime(i32)
}


//A partial field of a word, with optional values instead

#[derive(Default)]
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
        dbg!(left_field);
        dbg!(right_field);

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
}


///A struct representing a MIX instruction, with format: M = indexed memory address,
///modifier = the modifier or field specification, opcode=the opcode
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
struct Instruction{
    M: u32,
    modifier: u8,
    opcode: u8,
}


fn call_instruction(mut virtual_machine: &VirtualMachine, instruction: Word) -> InstructionResult{
    let instruction = load_instruction(virtual_machine, &instruction);
    match instruction.modifier{
        0=>virtual_machine.NOP(instruction.M, instruction.modifier),
    }
}
//Execute the instruction specified by the word, and return either a halt or the execution time
fn load_instruction(virtual_machine: &VirtualMachine, instruction: &Word) -> Instruction{
    let sign = instruction.is_negative;
    let addr_byte_one = instruction.byte_1;
    let addr_byte_two = instruction.byte_2;
    
    
    let memory = TwoByteWord{is_negative: sign, byte_1: addr_byte_one, byte_2: addr_byte_two};

    let index = instruction.byte_3;
    let modifier = instruction.byte_4;
    let opcode = instruction.byte_5;

    let byte_size = virtual_machine.byte_size;
    let index_register_value = match index{
        0 => 0,
        1 => virtual_machine.rI1.get_value(byte_size),
        2 => virtual_machine.rI2.get_value(byte_size),
        3 => virtual_machine.rI3.get_value(byte_size),
        4 => virtual_machine.rI4.get_value(byte_size),
        5 => virtual_machine.rI5.get_value(byte_size),
        6 => virtual_machine.rI6.get_value(byte_size),
        _ => panic!("Invalid index")
    } as i32;
   
    Instruction{M: (memory.get_value(byte_size) + index_register_value) as u32, modifier, opcode}

}



#[allow(non_snake_case)]


///Implementation for the MIX ISA. Every instruction depends on an opcode, indexed
///adress, and modification field. It is assumed that the address has already been indexed.
impl VirtualMachine{
    pub fn NOP(self, address: u32, field: u8){

    }

    pub fn ADD(self, address: u32, field: u8){

    }

    pub fn SUB(self, address: u32, field: u8){

    }
    pub fn MUL(self, address: u32, field: u8){

    }
    pub fn DIV(self, address: u32, field: u8){

    }
    pub fn special(self, address: u32, field: u8){

    }
    pub fn shift(self, address: u32, field: u8){

    }
    pub fn MOVE(self, address: u32, field: u8){

    }
    pub fn LDA(self, address: u32, field: u8){

    }
    pub fn LD1(self, address: u32, field: u8){

    }
    pub fn LD2(self, address: u32, field: u8){

    }
    pub fn LD3(self, address: u32, field: u8){

    }
    pub fn LD4(self, address: u32, field: u8){

    }
    pub fn LD5(self, address: u32, field: u8){

    }
    pub fn LD6(self, address: u32, field: u8){

    }
    pub fn LDX(self, address: u32, field: u8){

    }
    pub fn LDAN(self, address: u32, field: u8){

    }
    pub fn LD1N(self, address: u32, field: u8){

    }
    pub fn LD2N(self, address: u32, field: u8){

    }
    pub fn LD3N(self, address: u32, field: u8){

    }
    pub fn LD4N(self, address: u32, field: u8){

    }
    pub fn LD5N(self, address: u32, field: u8){

    }
    pub fn LD6N(self, address: u32, field: u8){

    }
    pub fn LDXN(self, address: u32, field: u8){

    }


    //Storing operators
    pub fn STA(self, address: u32, field: u8){

    }
    pub fn ST1(self, address: u32, field: u8){

    }
    pub fn ST2(self, address: u32, field: u8){

    }
    pub fn ST3(self, address: u32, field: u8){

    }
    pub fn ST4(self, address: u32, field: u8){

    }
    pub fn ST5(self, address: u32, field: u8){

    }
    pub fn ST6(self, address: u32, field: u8){

    }
    pub fn STX(self, address: u32, field: u8){

    }
    pub fn STZ(self, address: u32, field: u8){

    }
    pub fn JBUS(self, address: u32, field: u8){

    }
    pub fn IOC(self, address: u32, field: u8){

    }
    pub fn IN(self, address: u32, field: u8){

    }
    pub fn OUT(self, address: u32, field: u8){

    }
   
    pub fn JRED(self, address: u32, field: u8){

    }
   
    pub fn jump(self, address: u32, field: u8){

    }
   
    pub fn JA(self, address: u32, field: u8){

    }
   
    pub fn J1(self, address: u32, field: u8){

    }
   
    pub fn J2(self, address: u32, field: u8){

    }
   
    pub fn J3(self, address: u32, field: u8){

    }
    pub fn J4(self, address: u32, field: u8){

    }
   
    pub fn J5(self, address: u32, field: u8){

    }
   
   
    pub fn J6(self, address: u32, field: u8){

    }
   
   
    pub fn JX(self, address: u32, field: u8){

    }

    pub fn increment_decrement_A(self, address: u32, field: u8){
    }
    pub fn increment_decrement_(self, address: u32, field: u8){
    }
    pub fn increment_decrement_A(self, address: u32, field: u8){
    }
    pub fn increment_decrement_A(self, address: u32, field: u8){
    }
    pub fn increment_decrement_A(self, address: u32, field: u8){
    }
   
   
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_partial_field() {
        // Create a Word instance for testing
        let original_word = Word {
            is_negative: true,
            byte_1: 10,
            byte_2: 20,
            byte_3: 30,
            byte_4: 40,
            byte_5: 50,
        };

        // Test case: valid field specification
        let field = 28; // Binary representation of 28
        let partial_field = PartialField::new(&original_word, field);

        assert_eq!(partial_field.is_negative, None);
        assert_eq!(partial_field.byte_1, None);
        assert_eq!(partial_field.byte_2, None);
        assert_eq!(partial_field.byte_3, Some(30)); 
        assert_eq!(partial_field.byte_4, Some(40)); 
        assert_eq!(partial_field.byte_5, None); 

    }

    #[test]
    fn test_load_instruction(){

        let m: Vec<Word> = Vec::new();
        let mut mock_vm = VirtualMachine::new(m, 0, 64);
        let mut word = Word{is_negative: false, byte_1: 0, byte_2: 0, byte_3: 0, byte_4: 0, byte_5: 0};
        let instr = load_instruction(&mock_vm, &word);
        let mut expected = Instruction{M: 0, modifier: 0, opcode: 0};
        assert_eq!(instr, expected);
        
        mock_vm.rI1.store_value(300, 64);

        word.is_negative = true;
        word.byte_1 = 3;
        word.byte_3 = 1;
        word.byte_4 = 50;
        word.byte_5 = 30;
        word.is_negative = true;
        let instr = load_instruction(&mock_vm, &word);
    
        expected.M = (-(3 * 64) + 300) as u32;
        expected.modifier = 50;
        expected.opcode = 30;
        assert_eq!(instr, expected);

    }
}
