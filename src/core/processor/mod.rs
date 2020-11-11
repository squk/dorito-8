use crate::core::display::Display;
use crate::core::memory::Memory;
use rand::{Rng, thread_rng};

pub struct Processor {
    PC: u16,
    I: u16,
    //stack: [u16; 16],
    stack: Vec<u16>,
    pub V: [u8; 16],
    pub Memory: Memory,
    pub Display: Display,
    pub delay: u8,
    pub sound: u8,
    pub draw_flag: bool,
}

impl Default for Processor {
    fn default() -> Processor {
        Processor {
            PC: 0x200,
            I: 0,
            //stack: [0; 16],
            stack: vec![],
            V: [0; 16],
            Memory: Memory::default(),
            Display: Display::default(),
            delay: 0,
            sound: 0,
            draw_flag: false,
        }
    }
}

impl Processor {
    // run a single CPU cycle
    pub fn step(&mut self) {
        // Fetch Opcode
        let op = self.Memory.read_u16(self.PC);
        //let op = 0xD111;
        // Decode and Execute Opcode
        self.decode_exec(op);
        // Decrement timers
        self.timers();
    }

    pub fn timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            //TODO emit sound
            self.sound -= 1;
        }
    }

    // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#decode
    fn decode_exec(&mut self, op: u16) {
        let t = (op & 0xF000) >> 12; // first nibble
        let x = (op & 0xF00) >> 8; // second nibble - used to look up one of the 16 registers
        let y = (op & 0xF0) >> 4; // third nibble - also used to look up one of the 16 registers
        let n = op & 0xF; // fourth nibble
        let nn = (op & 0xFF) as u8; // second byte - 8-bit immediate number
        let nnn = op & 0xFFF; // second, third and fourth nibbles - 12-bit immediate memory address.

        //convinient for accessing V register
        let ix = x as usize;
        let iy = y as usize;

        match t {
            0x0 => {
                match nnn {
                    0x0E0 => {
                        self.draw_flag = true;
                        self.Display.clear();
                    }
                    0x0EE => {
                        // return from subroutine;
                        let sp = self.stack.pop();
                        if !sp.is_none() {
                            self.PC = sp.unwrap();
                        }
                    }  
                    _ => {}      // call - 0NNN
                }
            }
            0x1 => { //  1NNN - goto NNN;
                self.goto(nnn);
            }
            0x2 => { //  2NNN - *(0xNNN)()
                self.stack.push(self.PC);
                self.goto(nnn);
            }
            0x3 => { // 3XNN - if(Vx==NN)
                //self.skip_on_equal(x, nn);
                if self.V[ix] == nn {
                    self.PC += 2;
                }
            }
            0x4 => { // 4XNN - if(Vx!=NN)
                //self.skip_on_unequal(x, nn);
                if self.V[ix] != nn {
                    self.PC += 2;
                }
            }
            0x5 => {
                match n {
                    0x0 => {// 5XY0 - if(Vx==Vy)
                        //let vy = self.V.read_u16(y);
                        //self.skip_on_equal(x, vy);
                        if self.V[ix] == self.V[iy] {
                            self.PC += 2;
                        }
                    }
                    _ =>{
                        println!("invalid opcode")
                    }
                }
            }
            0x6 => { // 6XNN - Vx = NN
                self.V[ix] = nn;
            }
            0x7 => { // 7XNN - Vx += NN
                self.V[ix] = self.V[ix] + nn;
            }
            0x8 => {
                // 8XY... bit ops and math
                match n {
                    0x0 => { // 8XY0 - Vx=Vy
                        self.V[ix] = self.V[iy];
                    }
                    0x1 => { // 8XY1 - Vx=Vx|Vy
                        let z = self.V[ix] | self.V[iy];

                        self.V[ix] = z;
                    }
                    0x2 => { // 8XY2 - Vx=Vx&Vy
                        let z = self.V[ix] & self.V[iy];

                        self.V[ix] = z;
                    }
                    0x3 => { // 8XY3 - Vx=Vx^Vy
                        let z = self.V[ix] ^ self.V[iy];

                        self.V[ix] = z;
                    }
                    0x4 => { // 8XY4 - Vx += Vy
                        let z = self.V[ix] + self.V[iy];
                        if (z) > 255 {
                            self.V[0xF] = 1;
                        } else {
                            self.V[0xF] = 0;
                        }
                        self.V[ix] = z & 0xFF;
                    }
                    0x5 => { // 8XY5 - Vx -= Vy
                        let z = self.V[ix] - self.V[iy];
                        if z >= 0 {
                            self.V[0xF] = 1;
                        } else {
                            self.V[0xF] = 0;
                        }
                        self.V[ix] = z & 0xFF;
                    }
                    0x6 => { // 8XY6 - Vx>>=1
                        //get least significant bit
                        let lsb = self.V[ix] & 0x1;
                        //f is our Special register
                        self.V[0xF] = lsb;

                        //self.V.shift_right(x);
                        self.V[ix] = self.V[ix] >> 1;
                    }
                    0x7 => { // 8XY7 - Vx=Vy-Vx
                        let z = self.V[iy] - self.V[ix];
                        if z >= 0 {
                            self.V[0xF] = 1;
                        } else {
                            self.V[0xF] = 0;
                        }
                        self.V[ix] = z & 0xFF;
                    }
                    0xE => { // 8XYE - Vx<<=1
                        //get most significant bit, 0x80 == 1000 0000
                        let msb = self.V[ix] & 0x80;
                        //f is our Special register
                        self.V[0xF] = msb;

                        //self.V.shift_left(x);
                        self.V[ix] = self.V[ix] << 1;
                    }
                    _ => println!("invalid opcode"),
                }
            }
            0x9 => {
                match n {
                    0x0 => { // 9XY0 - if(Vx!=Vy)
                        //skip_on_unequal
                        if self.V[ix] != self.V[iy] {
                            self.PC += 2;
                        }
                    }
                    _ => println!("invalid opcode"),
                }
            }
            0xA => { // ANNN - I = NNN
                self.I = nnn;
            }
            0xB => { // BNNN - PC=V0+NNN
                let address = self.V[0] as u16 + nnn;
                self.goto(address);
            }
            0xC => { // CXNN - Vx=rand()&NN
                let mut rng = thread_rng();
                let z: u8 = rng.gen::<u8>() & nn;
                self.V[ix] = z;
            }
            0xD => { // DXYN - draw(Vx,Vy,N)
                // set draw flag to refresh screen and reset F register
                println!("DXYN");
                self.V[0xF] = 0;
                self.draw_flag = true;
                // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
                // for n rows of sprite data
                for yline in 0..n {
                    //get sprite data from memory
                    let row = self.Memory.read_u8(self.I + yline);
                    let yc = (self.V[iy] + yline as u8) % 32;//default height
                    for xline in 0..8 {
                        let xc = (self.V[ix] + xline) % 64;//default width
                        let fb = ((yc * 32) + xc) as usize;
                        let pixel = self.Display.frame_buffer[fb];
                        //flip pixel
                        if pixel == 1 {
                            self.V[0xF] = 1;
                            self.Display.frame_buffer[fb] = 0;
                        } else {
                            self.Display.frame_buffer[fb] = 1;
                        }
                    }
                }
                self.PC += 2;
            }
            0xE => {
                match nn {
                    0x9E => { // EX9E - if(key()==Vx)
                    }
                    0xA1 => { // EXA1 - if(key()!=Vx)
                    }
                    _ => println!("invalid opcode"),
                }
            }
            0xF => {
                match nn {
                    0x07 => { // FX07 - Vx = get_delay()
                    }
                    0x0A => { // FX0A - Vx = get_key()
                    }
                    0x15 => { // FX15 - delay_timer(Vx)
                        self.delay = self.V[ix];
                    }
                    0x18 => { // FX18 - sound_timer(Vx)
                        self.sound = self.V[ix];
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
                    _ => println!("invalid opcode"),
                }
            }
            _ => println!("invalid opcode"),
        }
    }

    fn execute_op() {}

    // 
    fn goto(&mut self, address: u16) {
        self.PC = address;
    }

    // 3XNN, 5XY0
    fn skip_on_equal(&mut self, register: u8, value: u8) {
        if self.V[register as usize] == value {
            self.PC += 2;
        }
    }

    // 4XNN
    pub fn skip_on_unequal(&mut self, register: u8, value: u8) {
        if self.V[register as usize] != value {
            self.PC += 2;
        }
    }

    fn add_to_register(&mut self, register: u8, value: u8) {
        let newVal: u8 = value + self.V[register as usize];
        self.V[register as usize] = newVal;
    }
}

