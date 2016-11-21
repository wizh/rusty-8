extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::env;
use std::io::prelude::*;

mod cpu;
mod apu;
mod keypad;
mod display;
mod emulator;

use emulator::Emulator;

fn main() {
    let rom_name = env::args().nth(1).expect("usage: cargo run <rom>");
    let mut rom_fd = File::open(&rom_name).expect("rom not found");

    let mut rom_buf = Vec::new();
    rom_fd.read_to_end(&mut rom_buf);

    println!("{:?}", rom_buf);

    let mut emulator = Emulator::new(rom_buf);
    emulator.run();
}