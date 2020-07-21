const RAM_SIZE: usize = 0x1000;

pub struct Processor {
    PC: u16,
    I: u16,
    V: [u8; 16],
    ram: [u8; RAM_SIZE],
}

impl Default for Processor {
    fn default() -> Processor {
        Processor {
            PC: 0x200,
            I: 0,
            V: [0; 16],
            ram: [0; RAM_SIZE],
        }
    }
}

impl Processor {
    // run a single CPU cycle
    pub fn step(&self) {
        // Fetch Opcode
         let op = self.ram[self.PC]
        // Decode Opcode
        // Execute Opcode
    }

    // 1NNN
    fn goto(&mut self, address: u16) {
        self.PC = address;
    }

    // 3XNN
    fn skip_on_equal(&mut self, register: u8, value: u8) {
        if self.V[register] == value {
            self.PC += 2;
        }
    }
}
