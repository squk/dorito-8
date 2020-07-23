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
            0xA =>  { // ANNN - I = NNN

            }
            0xB =>  { // BNNN - PC=V0+NNN

            }
            0xC => { // CXNN - Vx=rand()&NN

            }
            0xD => { // DXYN - draw(Vx,Vy,N)

            }
            0xE => {
                match nn {
                    0x9E  => { // EX9E - if(key()==Vx)
                    }
                    0xA1 => { // EXA1 - if(key()!=Vx)
                    }
                    _ => {
                        println!("invalid opcode")
                    }
                }
            }
            0xF => {
                match nn {
                    0x07 => { // FX07 - Vx = get_delay()
                    }
                    0x0A => { // FX0A - Vx = get_key()
                    }
                    0x15 => { // FX15 - delay_timer(Vx)
                    }
                    0x18 => { // FX18 - sound_timer(Vx)
                    }
                    0x1E => { // FX1E - I +=Vx
                        // Most CHIP-8 interpreters' FX1E instructions do not affect VF, with one
                        // exception: The CHIP-8 interpreter for the Commodore Amiga sets VF to 1
                        // when there is a range overflow (I+VX>0xFFF), and to 0 when there
                        // isn't.[13] The only known game that depends on this behavior is
                        // Spacefight 2091! while at least one game, Animal Race, depends on VF
                        // not being affected.
                    }
                    0x29 => { // FX29 - I=sprite_addr[Vx]
                    }
                    0x33 => { // FX33 - set_BCD(Vx); *(I+0)=BCD(3); *(I+1)=BCD(2); *(I+2)=BCD(1);
                    }
                    0x55 => { // FX55 - reg_dump(Vx,&I)
                    }
                    0x65 => { // FX65 - reg_load(Vx,&I)
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
