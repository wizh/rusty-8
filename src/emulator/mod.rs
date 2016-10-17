mod cpu;
mod gpu;
mod apu;

use self::cpu::CPU;
use self::gpu::GPU;
use self::apu::APU;

const CLOCK_RATE: u32 = 600;
const FPS: u32 = 60;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 640;

pub struct Emulator {
    cpu: CPU,
    gpu: GPU,
    apu: APU,

    clock_rate: u32,
    fps: u32
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Emulator {
        Emulator {
            cpu: CPU::new(rom),
            gpu: GPU::new(WIDTH, HEIGHT),
            apu: APU::new(),
            clock_rate: CLOCK_RATE,
            fps: FPS,
        }
    }
}