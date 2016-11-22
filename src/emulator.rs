extern crate sdl2;

use cpu::CPU;
use apu::APU;
use keypad::Keypad;
use display::Display;

pub const PROGRAM_OFFSET: usize = 0x200;
pub const NUM_STACK_FRAMES: usize = 16;
pub const NUM_REGS: usize = 16;

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 640;
pub const SCALE: u32 = 20;

pub const NUM_KEYS: usize = 0xf;

const CLOCK_RATE: u32 = 600;
const FPS: u32 = 60;

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

    clock_rate: u32,
    fps: u32,
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
            clock_rate: CLOCK_RATE,
            fps: FPS,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.tick();
            self.display.draw(&self.cpu.g_mem);
        }
    }
}