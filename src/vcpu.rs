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

        //Sentinel value for halt instruction
        if res = -1 {
            execution_time += 1;
            break
        }
        else{
            execution_time += res;
        }

    }
}


//A partial field of a word, with optional values instead




///A struct representing a MIX instruction, with format: M = indexed memory address,
///modifier = the modifier or field specification, opcode=the opcode
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
struct Instruction{
    M: u32,
    modifier: u8,
    opcode: u8,
}



///Switching table to call instruction based on opcode
///Returns either an execution time or -1
///Potential optimization: hash the opcode field pair to call instructions 
///defined by partial fields in one instruction
fn call_instruction(mut virtual_machine: &VirtualMachine, instruction: Word) -> i32{
    let instruction = load_instruction(virtual_machine, &instruction);
    match instruction.modifier{
        0=>virtual_machine.NOP(instruction.M, instruction.modifier),
        1=>virtual_machine.ADD(instruction.M, instruction.modifier),
        2=>virtual_machine.SUB(instruction.M, instruction.modifier),
        3=>virtual_machine.MUL(instruction.M, instruction.modifier),
        4=>virtual_machine.DIV(instruction.M, instruction.modifier),
        5=>virtual_machine.special(instruction.M, instruction.modifier),
        6=>virtual_machine.shift(instruction.M, instruction.modifier),
        7=>virtual_machine.MOVE(instruction.M, instruction.modifier),
        8=>virtual_machine.LDA(instruction.M, instruction.modifier),
        9=>virtual_machine.LD1(instruction.M, instruction.modifier),
        10=>virtual_machine.LD2(instruction.M, instruction.modifier),
        11=>virtual_machine.LD3(instruction.M, instruction.modifier),
        12=>virtual_machine.LD4(instruction.M, instruction.modifier),
        13=>virtual_machine.LD5(instruction.M, instruction.modifier),
        14=>virtual_machine.LD6(instruction.M, instruction.modifier),
        15=>virtual_machine.LDX(instruction.M, instruction.modifier),
        16=>virtual_machine.LDAN(instruction.M, instruction.modifier),
        17=>virtual_machine.LD1N(instruction.M, instruction.modifier),
        18=>virtual_machine.LD2N(instruction.M, instruction.modifier),
        19=>virtual_machine.LD3N(instruction.M, instruction.modifier),
        20=>virtual_machine.LD4N(instruction.M, instruction.modifier),
        21=>virtual_machine.LD5N(instruction.M, instruction.modifier),
        22=>virtual_machine.LD6N(instruction.M, instruction.modifier),
        23=>virtual_machine.LDXN(instruction.M, instruction.modifier),
        24=>virtual_machine.STA(instruction.M, instruction.modifier),
        25=>virtual_machine.ST1(instruction.M, instruction.modifier),
        26=>virtual_machine.ST2(instruction.M, instruction.modifier),
        27=>virtual_machine.ST3(instruction.M, instruction.modifier),
        28=>virtual_machine.ST4(instruction.M, instruction.modifier),
        29=>virtual_machine.ST5(instruction.M, instruction.modifier),
        30=>virtual_machine.ST6(instruction.M, instruction.modifier),
        31=>virtual_machine.STX(instruction.M, instruction.modifier),
        32=>virtual_machine.STJ(instruction.M, instruction.modifier),
        33=>virtual_machine.STZ(instruction.M, instruction.modifier),
        34=>virtual_machine.JBUS(instruction.M, instruction.modifier),
        35=>virtual_machine.IOC(instruction.M, instruction.modifier),
        36=>virtual_machine.IN(instruction.M, instruction.modifier),
        37=>virtual_machine.OUT(instruction.M, instruction.modifier),
        38=>virtual_machine.JRED(instruction.M, instruction.modifier),
        39=>virtual_machine.jump(instruction.M, instruction.modifier),
        40=>virtual_machine.JA(instruction.M, instruction.modifier),
        41=>virtual_machine.J1(instruction.M, instruction.modifier),
        42=>virtual_machine.J2(instruction.M, instruction.modifier),
        43=>virtual_machine.J3(instruction.M, instruction.modifier),
        44=>virtual_machine.J4(instruction.M, instruction.modifier),
        45=>virtual_machine.J5(instruction.M, instruction.modifier),
        46=>virtual_machine.J6(instruction.M, instruction.modifier),
        47=>virtual_machine.JX(instruction.M, instruction.modifier),
        48=>virtual_machine.increment_decrement_A(instruction.M, instruction.modifier),
        49=>virtual_machine.increment_decrement_1(instruction.M, instruction.modifier),
        50=>virtual_machine.increment_decrement_2(instruction.M, instruction.modifier),
        51=>virtual_machine.increment_decrement_3(instruction.M, instruction.modifier),
        52=>virtual_machine.increment_decrement_4(instruction.M, instruction.modifier),
        53=>virtual_machine.increment_decrement_5(instruction.M, instruction.modifier),
        54=>virtual_machine.increment_decrement_6(instruction.M, instruction.modifier),
        55=>virtual_machine.increment_decrement_X(instruction.M, instruction.modifier),
        56=>virtual_machine.CMPA(instruction.M, instruction.modifier),
        57=>virtual_machine.CMP1(instruction.M, instruction.modifier),
        58=>virtual_machine.CMP2(instruction.M, instruction.modifier),
        59=>virtual_machine.CMP3(instruction.M, instruction.modifier),
        60=>virtual_machine.CMP4(instruction.M, instruction.modifier),
        61=>virtual_machine.CMP5(instruction.M, instruction.modifier),
        62=>virtual_machine.CMP6(instruction.M, instruction.modifier),
        63=>virtual_machine.CMPX(instruction.M, instruction.modifier),
        _=>panic!("Invalid opcode"),
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





///Implementation for the MIX ISA. Every instruction depends on an opcode, indexed
///adress, and modification field. It is assumed that the address has already been indexed.
///There's probably some better way to do this, maybe involving traits
#[allow(non_snake_case)]
impl VirtualMachine{
    pub fn NOP(&mut self, address: u32, field: u8) -> i32{
        1
    }

   pub fn ADD(&mut self, address: u32, field: u8) -> i32{
       let v = self.load_v(address, field).unwrap();
       let res = self.get_rA_val() + v;
       self.rA.store_value(res, self.byte_size);
       2
    }

    pub fn SUB(&mut self, address: u32, field: u8) -> i32{
       let mut v = self.get_word(address).unwrap();
       v.is_negative = !v.is_negative;
       self.rA.add(&v, self.byte_size);
       2
    }
    
    
    pub fn MUL(&mut self, address: u32, field: u8) -> i32{
        let v = self.get_word(address).unwrap().get_value(self.byte_size) as i64;
        let max_word_val: i64 = self.byte_size.pow(4) as i64 - 1;
        let rax_val: i64 = (v * self.rA.get_value(self.byte_size) as i64);
        let rx_val = (rax_val % max_word_val) as i32;
        let ra_val = ((rax_val - rx_val) / max_word_val) as i32;
        self.rA.store_value(ra_val, self.byte_size);
        self.rX.store_value(rx_val, self.byte_size);
        5
        
    }
    
    pub fn DIV(&mut self, address: u32, field: u8) -> i32{
        let rax_val = self.rX.get_value(self.byte_size) + (self.rA.get_value(self.byte_size) * self.byte_size.pow(4));
        let quotient = rax_val/self.get_word(address).unwrap().get_value(self.byte_size);
        10
    }

    pub fn special(&mut self, address: u32, field: u8) -> i32{
        //TODO: Stub
        3
    }

    pub fn shift(&mut self, address: u32, field: u8) -> i32{

    }

    pub fn MOVE(&mut self, address: u32, field: u8) -> i32{

    }

    pub fn LDA(&mut self, address: u32, field: u8) -> i32{

    }

    pub fn LD1(&mut self, address: u32, field: u8) -> i32{

    }
    
    pub fn LD2(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD3(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD4(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD5(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD6(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LDX(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LDAN(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD1N(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD2N(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD3N(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD4N(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD5N(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LD6N(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn LDXN(&mut self, address: u32, field: u8) -> i32{

    }


    //Storing operators
    pub fn STA(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn ST1(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn ST2(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn ST3(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn ST4(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn ST5(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn ST6(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn STX(&mut self, address: u32, field: u8) -> i32{

    }

    pub fn STJ(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn STZ(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn JBUS(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn IOC(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn IN(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn OUT(&mut self, address: u32, field: u8) -> i32{

    }
   
    pub fn JRED(&mut self, address: u32, field: u8) -> i32{

    }
   
    pub fn jump(&mut self, address: u32, field: u8) -> i32{

    }
   
    pub fn JA(&mut self, address: u32, field: u8) -> i32{

    }
   
    pub fn J1(&mut self, address: u32, field: u8) -> i32{

    }
   
    pub fn J2(&mut self, address: u32, field: u8) -> i32{

    }
   
    pub fn J3(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn J4(&mut self, address: u32, field: u8) -> i32{

    }
   
    pub fn J5(&mut self, address: u32, field: u8) -> i32{

    }
   
   
    pub fn J6(&mut self, address: u32, field: u8) -> i32{

    }
   
   
    pub fn JX(&mut self, address: u32, field: u8) -> i32{

    }

    pub fn increment_decrement_A(&mut self, address: u32, field: u8) -> i32{
    }

    pub fn increment_decrement_1(&mut self, address: u32, field: u8) -> i32{
    }
    
    pub fn increment_decrement_2(&mut self, address: u32, field: u8) -> i32{
    }

    pub fn increment_decrement_3(&mut self, address: u32, field: u8) -> i32{
    }

    pub fn increment_decrement_4(&mut self, address: u32, field: u8) -> i32{
    }
    
    pub fn increment_decrement_5(&mut self, address: u32, field: u8) -> i32{
    }

    pub fn increment_decrement_6(&mut self, address: u32, field: u8) -> i32{
    }
    
    pub fn increment_decrement_X(&mut self, address: u32, field: u8) -> i32{
    }

    pub fn CMPA(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn CMP1(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn CMP2(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn CMP3(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn CMP4(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn CMP5(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn CMP6(&mut self, address: u32, field: u8) -> i32{

    }
    pub fn CMPX(&mut self, address: u32, field: u8) -> i32{

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
