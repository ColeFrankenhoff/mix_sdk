#[derive(Debug, PartialEq, Eq, Default)]
pub struct Word{
    is_negative: bool,
    byte_0: u8,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    byte_4: u8,
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
