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
         self.decode_exec(op)
    }

    // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#decode
    fn decode_exec(&self, op: u16) {
        let t = op & 0xF000; // first nibble
        let x = op & 0xF00; // second nibble - used to look up one of the 16 registers
        let y = op & 0xF0; // third nibble - also used to look up one of the 16 registers
        let n = op & 0xF; // fourth nibble
        let nn = op & 0xFF; // second byte - 8-bit immediate number
        let nnn = op & 0xFFF; // second, third and fourth nibbles - 12-bit immediate memory address.

        match t {
            0x0 => {
                match op {
                    0x00E0 => {} // disp_clear()
                    0x0EE => {} // return;
                    _ => {} // call - 0NNN
                }
            }
            0x1 =>  { //  1NNN - goto NNN;

            }
            0x2 =>  { //  2NNN - *(0xNNN)()

            }
            0x3 => { // 3XNN - if(Vx==NN)

            }
            0x4 => { // 4XNN - if(Vx!=NN)

            }
            0x5 => {
                if n == 0x0  {// 5XY0 - if(Vx==Vy)
                } else {
                    println!("invalid opcode")
                }
            }
            0x6 => { // 6XNN - Vx = NN

            }
            0x7 => { // 7XNN - Vx += NN

            }
            0x8 => { // 8XY... bit ops and math
                match n {
                    0x0 => { // 8XY0 - Vx=Vy

                    }
                    0x1 => { // 8XY1 - Vx=Vx|Vy

                    }
                    0x2 => { // 8XY2 - Vx=Vx&Vy

                    }
                    0x3 => { // 8XY3 - Vx=Vx^Vy

                    }
                    0x4 => { // 8XY4 - Vx += Vy

                    }
                    0x5 => { // 8XY5 - Vx -= Vy

                    }
                    0x6 => { // 8XY6 - Vx>>=1

                    }
                    0x7 => { // 8XY7 - Vx=Vy-Vx

                    }
                    0xE => { // 8XYE - Vx<<=1
                    }
                    _ => {
                        println!("invalid opcode")
                    }
                }
            }
        }
    }

    fn execute_op() {

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
