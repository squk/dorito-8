const ROM_INDEX: usize = 0x200;
const RAM_SIZE: usize = 0x1000;
const FONT_INDEX: usize = 0x50;
const FONT_SIZE: usize = 0x50;

pub struct Memory {
    ram: [u8; RAM_SIZE],
}

impl Default for Memory {
    fn default() -> Memory {
        let mut m = Memory { ram: [0; RAM_SIZE] };

        let fontset: Vec<u8> = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        m.write_bytes(FONT_INDEX as u16, fontset);

        return m;
    }
}

impl Memory {
    // read a byte from an address
    pub fn read_u8(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    // read two bytes from an address
    pub fn read_u16(&self, address: u16) -> u16 {
        let b1: u16 = (self.ram[address as usize] as u16) << 8;
        let b2: u16 = self.ram[(address + 1) as usize] as u16;
        b1 | b2
    }

    // write a byte to an address
    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    // write 2 bytes to an address
    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.ram[address as usize] = ((value & 0xFF00) >> 8) as u8;
        self.ram[(address + 1) as usize] = (value & 0xFF) as u8;
    }

    pub fn write_bytes(&mut self, address: u16, bytes: Vec<u8>) {
        for i in 0..bytes.len() {
            self.write_u8(address, bytes[i])
        }
    }

    pub fn load_rom(&mut self, filename: String) {
        match std::fs::read(filename) {
            Ok(bytes) => {
                if bytes.len() > (RAM_SIZE - ROM_INDEX) {
                    panic!("rom too large") // TODO: don't panic
                }

                for i in 0..bytes.len() {
                    self.ram[ROM_INDEX + i] = bytes[i]
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("please run again with appropriate permissions.");
                    return;
                }
                panic!("{}", e);
            }
        }
    }

    pub fn shift_right(&mut self, register: u16) {
        // self.ram[register] >> 1;
    }

    pub fn shift_left(&mut self, register: u16) {
        // self.ram[register] << 1;
    }
}
