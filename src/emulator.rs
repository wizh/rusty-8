use cpu::CPU;
use display::Display;
use apu::APU;
use keypad::Keypad;

pub const PROGRAM_OFFSET: usize = 0x200;
pub const NUM_STACK_FRAMES: usize = 16;
pub const NUM_REGS: usize = 16;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

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

pub struct Emulator {
    cpu: CPU,
    display: Display,
    apu: APU,
    keypad: Keypad,

    clock_rate: u32,
    fps: u32,
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Emulator {
        let mut memory = vec![0; MEM_SIZE];

        for i in 0..FONTSET.len() {
            memory[i] = FONTSET[i];
        }

        for i in 0..rom.len() {
            memory[i + PROGRAM_OFFSET] = rom[i];
        }

        Emulator {
            cpu: CPU::new(memory),
            display: Display::new(WIDTH, HEIGHT),
            apu: APU::new(),
            keypad: Keypad::new(),
            clock_rate: CLOCK_RATE,
            fps: FPS,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.tick();
            self.display.draw();
            self.keypad.set_keys()
        }
    }
}