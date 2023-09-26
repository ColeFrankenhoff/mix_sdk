//!Define a single public function 'run', which takes a virtual_machine as an argument 
//!and executes the code. Stops after a reasonable amount of MIX units of time
use crate::virtual_machine::VirtualMachine;
use crate::virtual_machine::Word;
use crate::virtual_machine::TwoByteWord;


///Execute the contents of a virtual machine, starting at the value specified by rsp
///
pub fn run_vm(virtual_machine: VirtualMachine){
    
    let mut execution_time = 0;
    let 
    loop{
        let rsp = virtual_machine.rsp;
        let w = virtual_machine.get_word(rsp);
    }
}

fn execute_instruction(virtual_machine: &mut VirtualMachine, instruction: Word){

}
]
