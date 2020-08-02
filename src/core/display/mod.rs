extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureCreator;
use std::path::Path;
use std::process;

const DEFAULT_WIDTH: u32 = 64;
const DEFAULT_HEIGHT: u32 = 32;

pub struct Display {
    pub width: u32,
    pub height: u32,
    display_buffer: Vec<u8>,

    ctx: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
}

impl Default for Display {
    fn default() -> Display {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();

        let gl_attr = video.gl_attr();
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        let window = match video
            .window("dorito-8", DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .position_centered()
            .resizable()
            .opengl()
            .build()
        {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err),
        };

        let canvas = window.into_canvas().build().unwrap();

        Display {
            display_buffer: vec![],
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,

            ctx: ctx,
            video: video,
            canvas: canvas,
        }
    }
}

impl Display {
    pub fn draw(&mut self) {
        let black = Color::RGB(0, 0, 0);
        let white = Color::RGB(255, 255, 255);
        let green = Color::RGB(0, 255, 0);
        let yellow = Color::RGB(255, 255, 0);
        let red = Color::RGB(255, 0, 0);
        let cyon = Color::RGB(0, 255, 255);
        let purple = Color::RGB(255, 0, 255);

        let mut events = self.ctx.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    process::exit(1);
                }
                _ => {}
            }
        }

        self.canvas.set_draw_color(black);
        self.canvas.clear();

        for x in 0..self.width {
            for y in 0..self.height {
                let mut color: Color;
                if (x % 2 == 0) && (y % 2 == 0) {
                    color = green;
                } else if (x % 2 == 1) && (y % 2 == 1) {
                    color = purple;
                } else {

                self.draw_px(x as i16, y as i16, color);
            }
        }

        self.canvas.set_draw_color(black);
        self.canvas.present();
    }

    // draws a single "pixel", actually just a rect
    pub fn draw_px(&mut self, x: i16, y: i16, color: Color) {
        let size = self.canvas.window().size();
        println!("size {:?}", size);
        let w = (size.0 as f32 / self.width as f32); // width ratio
        println!("{}", w);

        let x1: i32 = (w * x as f32) as i32;
        let x2: u32 = x1 as u32 + w as u32;
        println!("{} {}", x1, x2);

        let y1: i32 = (w * y as f32) as i32;
        let y2: u32 = y1 as u32 + w as u32;

        //let _ = self.canvas.rectangle(x1, y1, x2, y2, color);
        self.canvas.set_draw_color(color);
        let _ = self.canvas.fill_rect(Rect::new(x1, y1, x2, y2));
    }
}
