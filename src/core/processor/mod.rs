const RAM_SIZE: usize = 0x1000;

pub struct Processor {
    PC: u16,
    I: u16,
    V: [u8; 16],
    ram: [u8; RAM_SIZE],
}

impl Processor {}

impl Default for Processor {
    fn default() -> Processor {
        Processor {
            PC: 0, //TODO: 0x200?
            I: 0,
            V: [0; 16],
            ram: [0; RAM_SIZE],
        }
    }
}
