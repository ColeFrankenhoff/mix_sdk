use std::env;
use Crate::mixexe::Word;
use mixexe::TwoByteWord;

use crate::mixexe::Word;

enum Comparison{
    Equal,
    Less,
    Greater
}

struct VirtualMachine{
    memory: Vec<Word>,
    rA: Word,
    rX: Word,
    rJ: TwoByteWord,
    rI1: TwoByteWord,
    rI2: TwoByteWord,
    rI3: TwoByteWord,
    rI4: TwoByteWord,
    rI5: TwoByteWord,
    rI6: TwoByteWord,
    overflow_toggle: bool,
    comparison_toggle: Comparison
}

impl VirtualMachine{
    fn new(memory: Vec<word>) -> VirtualMachine{
        let rA = Word::default();
        let rX = Word::default();
        let rJ = Word::default();
        let rI1 = TwoByteWord::default();
        let rI2 = TwoByteWord::default();
        let rI3 = TwoByteWord::default();
        let rI4 = TwoByteWord::default();
        let rI5 = TwoByteWord::default();
        let rI6 = TwoByteWord::default();
        let rA = rA.zero();
        let rX = rX.zero();
        let rJ = rJ.zero();
        let rI1 = rI1.zero();
        let rI2 = rI2.zero();
        let rI3 = rI3.zero();

        VirtualMachine{
            memory,
            rA,
            rX,
            rJ,
            rI1,
            rI2,
            rI3,
            rI4,
        }
    }
}



//
pub fn run_vm(&)
