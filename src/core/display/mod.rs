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
    }
}
