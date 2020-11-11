use crate::core::processor::Processor;

mod core;

fn main() {
    let mut p = Processor::default();
    //p.Memory.load_rom(String::from("roms/PONG"));
    p.Memory.load_rom(String::from("roms/BC_test.ch8"));
    //p.Memory.load_rom(String::from("roms/test_opcode.ch8"));

    loop {
        p.step();
        if p.draw_flag {
            p.Display.draw();
            p.draw_flag = false;
        }
    }
}
