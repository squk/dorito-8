extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::core::processor::Processor;
use std::{thread, time};
use std::process;

mod core;

fn main() {
    let mut p = Processor::default();
    let tm = time::Duration::from_millis(10);

    //p.Memory.load_rom(String::from("roms/PONG"));
    p.Memory.load_rom(String::from("roms/pong.ch8"));
    //p.Memory.load_rom(String::from("roms/BC_test.ch8"));
    //p.Memory.load_rom(String::from("roms/test_opcode.ch8"));

    let mut events = p.Display.ctx.event_pump().unwrap();

    loop {
        //p.Display.drwTest();
        thread::sleep(tm);
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    p.key[0] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    p.key[1] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    p.key[2] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                    p.key[3] = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    p.key[4] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    p.key[5] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    p.key[6] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    p.key[7] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    p.key[8] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    p.key[9] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    p.key[10] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    p.key[11] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    p.key[12] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    p.key[13] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    p.key[14] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    p.key[15] = true;
                },
                _ => {}
            }
        }

        p.step();
        if p.draw_flag {
            p.Display.draw();
            p.draw_flag = false;
        }
    }
}
