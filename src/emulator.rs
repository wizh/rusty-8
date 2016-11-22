extern crate sdl2;

use cpu::CPU;
use apu::APU;
use keypad::Keypad;
use display::Display;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub const PROGRAM_OFFSET: usize = 0x200;
pub const NUM_STACK_FRAMES: usize = 16;
pub const NUM_REGS: usize = 16;

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 640;
pub const SCALE: u32 = 20;

pub const NUM_KEYS: usize = 0xf;

const CLOCK_RATE: u32 = 600;
const FPS: u32 = 60;

const DELAYTIMER: u32 = 60;

const MEM_SIZE: usize = 4 * 1024;

const FONTSET: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0,
                           0x20, 0x60, 0x20, 0x20, 0x70,
                           0xF0, 0x10, 0xF0, 0x80, 0xF0,
                           0xF0, 0x10, 0xF0, 0x10, 0xF0,
                           0x90, 0x90, 0xF0, 0x10, 0x10,
                           0xF0, 0x80, 0xF0, 0x10, 0xF0,
                           0xF0, 0x80, 0xF0, 0x90, 0xF0,
                           0xF0, 0x10, 0x20, 0x40, 0x40,
                           0xF0, 0x90, 0xF0, 0x90, 0xF0,
                           0xF0, 0x90, 0xF0, 0x10, 0xF0,
                           0xF0, 0x90, 0xF0, 0x90, 0x90,
                           0xE0, 0x90, 0xE0, 0x90, 0xE0,
                           0xF0, 0x80, 0x80, 0x80, 0xF0,
                           0xE0, 0x90, 0x90, 0x90, 0xE0,
                           0xF0, 0x80, 0xF0, 0x80, 0xF0,
                           0xF0, 0x80, 0xF0, 0x80, 0x80];

pub struct Emulator<'a> {
    cpu: CPU,
    display: Display<'a>,
    apu: APU,
    keypad: Keypad,
    sdl_context: sdl2::Sdl,
}

impl<'a> Emulator<'a> {
    pub fn new(rom: Vec<u8>) -> Emulator<'a> {
        let mut memory = vec![0; MEM_SIZE];

        for i in 0..FONTSET.len() {
            memory[i] = FONTSET[i];
        }

        for i in 0..rom.len() {
            memory[i + PROGRAM_OFFSET] = rom[i];
        }

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        Emulator {
            cpu: CPU::new(memory),
            display: Display::new(video_subsystem, WIDTH, HEIGHT),
            apu: APU::new(),
            keypad: Keypad::new(),
            sdl_context: sdl_context
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let timer_delay = Duration::new(0, 1);
        let frame_delay = Duration::new(0, 1);
        let cycle_delay = Duration::new(0, 1);

        let mut last_timer = Instant::now();
        let mut last_frame = Instant::now();
        let mut last_cycle = Instant::now();

        'emulation: loop {
            let current_time = Instant::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                        break 'emulation,
                    Event::KeyDown { keycode: Some(key), .. } =>
                        self.cpu.keypad.key_pressed(key),
                    Event::KeyUp { keycode: Some(key), .. } =>
                        self.cpu.keypad.key_released(key),
                    _ => {}
                }
            }

            if current_time.duration_since(last_timer) > timer_delay {
                if self.cpu.delay_timer_reg > 0 {
                    self.cpu.delay_timer_reg -= 1;
                }

                last_timer = Instant::now();
            }

            if current_time.duration_since(last_frame) > frame_delay {
                self.display.draw(&self.cpu.g_mem);
                last_frame = Instant::now();
            }

            if current_time.duration_since(last_cycle) > cycle_delay {
                self.cpu.tick();
                last_cycle = Instant::now();
            }

        }
    }
}