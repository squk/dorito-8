use crate::core::display::Display;
use crate::core::memory::Memory;
use rand::{Rng, thread_rng};
use std::collections::HashMap;

type Opcode = fn(&mut Processor, u16);
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
    pub key: [bool; 16],
    pub opTable: HashMap<u8, Opcode>,
    pub opTable0: HashMap<u8, Opcode>,
    pub opTable8: HashMap<u8, Opcode>,
    pub opTableE: HashMap<u8, Opcode>,
    pub opTableF: HashMap<u8, Opcode>,
}

impl Default for Processor {

    fn default() -> Processor {
        let mut ot = HashMap::new();
        ot.insert(0x0, (Processor::table0) as Opcode);
        ot.insert(0x1, (Processor::o_1NNN) as Opcode);
        ot.insert(0x2, (Processor::o_2NNN) as Opcode);
        ot.insert(0x3, (Processor::o_3XNN) as Opcode);
        ot.insert(0x4, (Processor::o_4XNN) as Opcode);
        ot.insert(0x5, (Processor::o_5XY0) as Opcode);
        ot.insert(0x6, (Processor::o_6XNN) as Opcode);
        ot.insert(0x7, (Processor::o_7XNN) as Opcode);
        ot.insert(0x8, (Processor::table8) as Opcode);
        ot.insert(0x9, (Processor::o_9XY0) as Opcode);
        ot.insert(0xA, (Processor::o_ANNN) as Opcode);
        ot.insert(0xB, (Processor::o_BNNN) as Opcode);
        ot.insert(0xC, (Processor::o_CXNN) as Opcode);
        ot.insert(0xD, (Processor::o_DXYN) as Opcode);
        ot.insert(0xE, (Processor::tableE) as Opcode);
        ot.insert(0xF, (Processor::tableF) as Opcode);
        let mut o0 = HashMap::new();
        o0.insert(0x0, (Processor::o_00E0) as Opcode);
        o0.insert(0xE, (Processor::o_00EE) as Opcode);
        let mut o8 = HashMap::new();
        o8.insert(0x0, (Processor::o_8XY0) as Opcode);
        o8.insert(0x1, (Processor::o_8XY1) as Opcode);
        o8.insert(0x2, (Processor::o_8XY2) as Opcode);
        o8.insert(0x3, (Processor::o_8XY3) as Opcode);
        o8.insert(0x4, (Processor::o_8XY4) as Opcode);
        o8.insert(0x5, (Processor::o_8XY5) as Opcode);
        o8.insert(0x6, (Processor::o_8XY6) as Opcode);
        o8.insert(0x7, (Processor::o_8XY7) as Opcode);
        o8.insert(0xE, (Processor::o_8XYE) as Opcode);
        let mut oE = HashMap::new();
        oE.insert(0x1, (Processor::o_EXA1) as Opcode);
        oE.insert(0xE, (Processor::o_EX9E) as Opcode);
        let mut oF = HashMap::new();
        oF.insert(0x07, (Processor::o_FX07) as Opcode);
        oF.insert(0x0A, (Processor::o_FX0A) as Opcode);
        oF.insert(0x15, (Processor::o_FX15) as Opcode);
        oF.insert(0x18, (Processor::o_FX18) as Opcode);
        oF.insert(0x1E, (Processor::o_FX1E) as Opcode);
        oF.insert(0x29, (Processor::o_FX29) as Opcode);
        oF.insert(0x33, (Processor::o_FX33) as Opcode);
        oF.insert(0x55, (Processor::o_FX55) as Opcode);
        oF.insert(0x65, (Processor::o_FX65) as Opcode);
        //TODO order members
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
            key: [false; 16],//key mapping in main
            opTable: ot,
            opTable0: o0,
            opTable8: o8,
            opTableE: oE,
            opTableF: oF
        }
    }
}

impl Processor {
    fn table0(&mut self, op: u16) {
        self.opTable0[&n(op)](self, op);
    }
    fn table8(&mut self, op: u16) {
        self.opTable8[&n(op)](self, op);
    }
    fn tableE(&mut self, op: u16) {
        self.opTableE[&n(op)](self, op);
    }
    fn tableF(&mut self, op: u16) {
        self.opTableF[&nn(op)](self, op);
    }
    // run a single CPU cycle
    pub fn step(&mut self) {
        // TODO find better solution
        if self.PC >= 0x1000 {
            self.PC = 0x200;
        }

        // Fetch Opcode
        let op = self.Memory.read_u16(self.PC);
        println!("{:#06x}", op);
        //println!("{:x}", self.PC);
        //let op = 0xD111;
        // Decode and Execute Opcode
        self.opTable[&t(op)](self, op);
        // Decrement timers
        self.timers();
        self.PC += 2;
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

    fn getKey(&mut self) -> u8 {
        for k in 0..16 {
            if self.key[k] {
                self.key[k] =  false;
                return k as u8;
            }
        }
        return 17;
    }

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

    // call
    fn o_0NNN(&mut self, address: u16) {
        self.PC = address;
    }

    // display clear
    fn o_00E0(&mut self, op: u16) {
        self.draw_flag = true;
        self.Display.clear();
    }

    // return from subroutine
    fn o_00EE(&mut self, op: u16) {
        let sp = self.stack.pop();
        if !sp.is_none() {
            self.PC = sp.unwrap();
        }
    }

    // goto nnn
    fn o_1NNN(&mut self, op: u16) {
        self.goto(nnn(op));
    }
    // call subroutine at nnn
    fn o_2NNN(&mut self, op: u16) {
        self.stack.push(self.PC);
        self.goto(nnn(op));
    }
    // skip if equal
    fn o_3XNN(&mut self, op: u16) {
        if self.V[x(op)] == nn(op) {
            self.PC += 2;
        }
    }
    fn o_4XNN(&mut self, op: u16) {
        if self.V[x(op)] != nn(op) {
            self.PC += 2;
        }
    }
    //skip if Vx==Vy
    fn o_5XY0(&mut self, op: u16) {
        //let vy = self.V.read_u16(y);
        //self.skip_on_equal(x, vy);
        if self.V[x(op)] == self.V[y(op)] {
            self.PC += 2;
        }
    }
    fn o_6XNN(&mut self, op: u16) {
        self.V[x(op)] = nn(op);
    }
    fn o_7XNN(&mut self, op: u16) {
        let xx = x(op);
        let vx = self.V[xx] as u16;
        self.V[xx] = (vx + nn(op) as u16) as u8;
    }

    fn o_8XY0(&mut self, op: u16) {
        self.V[x(op)] = self.V[y(op)];
    }

    fn o_8XY1(&mut self, op: u16) {
        let xx = x(op);
        let z = self.V[xx] | self.V[y(op)];
        self.V[xx] = z;
    }

    fn o_8XY2(&mut self, op: u16) {
        let xx = x(op);
        let z = self.V[xx] & self.V[y(op)];
        self.V[xx] = z;
    }

    fn o_8XY3(&mut self, op: u16) {
        let xx = x(op);
        let z = self.V[xx] ^ self.V[y(op)];
        self.V[xx] = z;
    }

    fn o_8XY4(&mut self, op: u16) {
        let xx = x(op);
        let z = self.V[xx] as u16 + self.V[y(op)] as u16;
        if (z) > 255 {
            self.V[0xF] = 1;
        } else {
            self.V[0xF] = 0;
        }
        self.V[xx] = z as u8 & 0xFF;
    }

    fn o_8XY5(&mut self, op: u16) {
        let xx = x(op);
        let vx = self.V[xx] as i16;
        let vy = self.V[y(op)] as i16;
        let z = (vx - vy) as i16;
        if z >= 0 {
            self.V[0xF] = 1;
        } else {
            self.V[0xF] = 0;
        }
        self.V[xx] = z as u8 & 0xFF;
    }

    fn o_8XY6(&mut self, op: u16) {
        let xx = x(op);
        //get least significant bit
        let lsb = self.V[xx] & 0x1;
        //f is our Special register
        self.V[0xF] = lsb;

        //self.V.shift_right(x);
        self.V[xx] = self.V[xx] >> 1;
    }

    fn o_8XY7(&mut self, op: u16) {
        let xx = x(op);
        let vx = self.V[xx] as i16;
        let vy = self.V[y(op)] as i16;
        let z = (vy - vx) as i16;
        if z >= 0 {
            self.V[0xF] = 1;
        } else {
            self.V[0xF] = 0;
        }
        self.V[xx] = z as u8 & 0xFF;
    }

    fn o_8XYE(&mut self, op: u16) {
        let xx = x(op);
        //get most significant bit, 0x80 == 1000 0000
        let msb = self.V[xx] & 0x80;
        //f is our Special register
        self.V[0xF] = msb;

        self.V[xx] = self.V[xx] << 1;
    }

    fn o_9XY0(&mut self, op: u16) {
        //skip_on_unequal
        if self.V[x(op)] != self.V[y(op)] {
            self.PC += 2;
        }
    }

    fn o_ANNN(&mut self, op: u16) {
        self.I = nnn(op);
    }

    fn o_BNNN(&mut self, op: u16) {
        let address = self.V[0] as u16 + nnn(op);
        self.goto(address);
    }

    fn o_CXNN(&mut self, op: u16) {
        let mut rng = thread_rng();
        let z: u8 = rng.gen::<u8>() & nn(op);
        self.V[x(op)] = z;
    }

    fn o_DXYN(&mut self, op: u16) {
        // set draw flag to refresh screen and reset F register
        //println!("DXYN");
        //println!("{:#06x}", op);
        self.V[0xF] = 0;
        self.draw_flag = true;
        // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
        // for n rows of sprite data
        for yline in 0..n(op) {
            //get sprite data from memory -- 8*y because 8 is width of 
            //sprite and we want to skip bytes we've already checked
            let row_byte = self.Memory.read_u8(self.I + (8*yline) as u16);
            // set y coordinate
            let yc = (self.V[y(op)] + yline as u8) % 32;//mod by hieght to wrap
            //All sprites are 8 wide
            for xline in 0..8 {
                // set x coordinate
                let xc = (self.V[x(op)] + xline) % 64;//mod by width to wrap
                let fb = (yc as usize * 32) + xc as usize;
                let pixel = row_byte & (0x80 >> xline);
                // if pixel is set, flip it
                // register colision if this pixel was already set
                if (pixel == 1) && self.Display.frame_buffer[fb] == 1 {
                    self.V[0xF] = 1;
                    self.Display.frame_buffer[fb] = 0;
                } else if pixel == 1 {
                    self.Display.frame_buffer[fb] = 1;
                }
            }
        }
        self.PC += 2;
    }

    // skip if key==Vx
    fn o_EX9E(&mut self, op: u16) {
        let k = self.V[x(op)];
        if self.key[k as usize] {
            self.PC += 2;
        }
    }

    fn o_EXA1(&mut self, op: u16) {
        let k = self.V[x(op)];
        if !self.key[k as usize] {
            self.PC += 2;
        }
    }

    fn o_FX07(&mut self, op: u16) {
        self.V[x(op)] = self.delay;
    }

    fn o_FX0A(&mut self, op: u16) {
        let k = self.getKey();
        if k != 17 {
            self.V[x(op)] = k;
        } else {
            self.PC -= 2;
        }
    }

    fn o_FX15(&mut self, op: u16) {
        self.delay = self.V[x(op)];
    }

    fn o_FX18(&mut self, op: u16) {
        self.sound = self.V[x(op)];
    }

    fn o_FX1E(&mut self, op: u16) {
        self.I = self.I + self.V[x(op)] as u16;
    }

    fn o_FX29(&mut self, op: u16) {
        //self.I = self.V[x] as u16;
        //TODO
        //self.I = self.Memory.FONT_INDEX + (5 * self.V[x]) as u16;
        self.I = 0x50 + (5 * self.V[x(op)]) as u16;
    }

    fn o_FX33(&mut self, op: u16) {
        let mut val = self.V[x(op)];
        let addr = self.I;
        self.Memory.write_u8(addr + 2, val%10);
        val /= 10;
        self.Memory.write_u8(addr + 1, val%10);
        val /= 10;
        self.Memory.write_u8(addr, val%10);
    }

    fn o_FX55(&mut self, op: u16) {
        let xx = x(op);
        for i in 0..=xx {
            self.Memory.write_u8(self.I + i as u16, self.V[i]);
        }
    }

    fn o_FX65(&mut self, op: u16) {
        let xx = x(op);
        for i in 0..=xx {
            self.V[i] = self.Memory.read_u8(self.I + i as u16);
        }
    }
}

#[inline(always)]
fn t(op: u16) -> u8 {
    ((op & 0xF000) >> 12) as u8
}
#[inline(always)]
fn x(op: u16) -> usize {
    ((op & 0xF00) >> 8) as usize
}
#[inline(always)]
fn y(op: u16) -> usize {
    ((op & 0xF0) >> 4) as usize
}
#[inline(always)]
fn n(op: u16) -> u8 {
    (op & 0xF) as u8
}
#[inline(always)]
fn nn(op: u16) -> u8 {
    (op & 0xFF) as u8
}
#[inline(always)]
fn nnn(op: u16) -> u16 {
    op & 0xFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testDecodeEx() {
        let mut p = Processor::default();
        let mut op = 0xD111;
        p.decode_exec(op);
        assert!(true);
    }
}
