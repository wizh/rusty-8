mod cpu;
mod gpu;
mod apu;
mod keypad;

use self::cpu::CPU;
use self::gpu::GPU;
use self::apu::APU;
use self::keypad::Keypad;

const CLOCK_RATE: u32 = 600;
const FPS: u32 = 60;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub struct Emulator {
    cpu: CPU,
    gpu: GPU,
    apu: APU,
    keypad: Keypad,

    clock_rate: u32,
    fps: u32,

    draw: bool,
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Emulator {
        Emulator {
            cpu: CPU::new(rom),
            gpu: GPU::new(WIDTH, HEIGHT),
            apu: APU::new(),
            keypad: Keypad::new(),
            clock_rate: CLOCK_RATE,
            fps: FPS,
            draw: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.tick();

            if self.draw {
                self.gpu.draw();
            }

            self.keypad.set_keys()
        }
    }
}