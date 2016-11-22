extern crate sdl2;

use emulator;

use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Display<'a> {
    height: u32,
    width: u32,
    renderer: sdl2::render::Renderer<'a>,
}

impl<'a> Display<'a> {
    pub fn new(video: sdl2::VideoSubsystem, width: u32, height: u32) -> Display<'a> {
        let window = video.window("Chip8", width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_scale((emulator::SCALE) as f32,
                           (emulator::SCALE) as f32);

        Display {
            height: height,
            width: width,
            renderer: renderer
        }
    }

    pub fn draw(&mut self, g_mem: &[[bool; 64]; 32]) {
        for y in 0..32 {
            for x in 0..64 {
                if g_mem[y][x] {
                    self.renderer.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.renderer.set_draw_color(Color::RGB(255, 255, 255));
                }

                self.renderer.draw_point(Point::new(x as i32, y as i32));
            }
        }

        self.renderer.present();
    }
}