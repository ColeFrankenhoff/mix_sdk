//!Define a single public function 'run', which takes a virtual_machine as an argument 
//!and executes the code. Stops after a reasonable amount of MIX units of time
use crate::virtual_machine::VirtualMachine;
use crate::virtual_machine::Word;
use crate::virtual_machine::TwoByteWord;



///Execute the contents of a virtual machine, starting at the value specified by rsp
///Take ownership of the vm's memory
pub fn run_vm(virtual_machine: VirtualMachine){
    
    let mut execution_time = 0;
    let vm_pointer = &mut virtual_machine;
    loop{
        let rsp = virtual_machine.rsp;
        let w = virtual_machine.get_word(rsp);

    }
}


//Each MIXAL instruction will either return an execution time or a halt
enum InstructionResult{
    HLT,
    ExecutionTime(i32)
}

fn execute_instruction(virtual_machine: &mut VirtualMachine, instruction: Word) -> InstructionResult {
    InstructionResult::HLT
}

