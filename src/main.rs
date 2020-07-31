use crate::core::processor::Processor;

mod core;

fn main() {
    let mut p = Processor::default();
    p.Memory.load_rom(String::from("roms/PONG"));

    loop {
        p.step();
    }
}
