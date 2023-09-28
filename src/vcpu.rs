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


//Execute the instruction specified by the word, and return either a halt or the execution time
fn call_instruction(mut virtual_machine: &VirtualMachine, instruction: Word) -> InstructionResult {
    let sign = instruction.is_negative;
    let addr_byte_one = instruction.byte_1;
    let addr_byte_two = instruction.byte_2;

    let index = instruction.byte_3;
    let modifier = instruction.byte_4;
    let opcode = instruction.byte_5;

    let index: TwoByteWord = match index{
        0 => 0,
        1 => virtual_machine.rI1,
        2 => virtual_machine.rI2,
        3 => virtual_machine.rI3,
        4 => virtual_machine.rI4,
        5 => virtual_machine.rI5,
        6 => virtual_machine.rI6,
        _ => panic!("Invalid index")
    };
    
    InstructionResult::HLT

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
}
