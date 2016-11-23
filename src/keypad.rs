extern crate sdl2;

use sdl2::keyboard::Keycode;

use emulator;

const KEY_1: u8 = 0x0;
const KEY_2: u8 = 0x1;
const KEY_3: u8 = 0x2;
const KEY_4: u8 = 0x3;

const KEY_Q: u8 = 0x4;
const KEY_W: u8 = 0x5;
const KEY_E: u8 = 0x6;
const KEY_R: u8 = 0x7;

const KEY_A: u8 = 0x8;
const KEY_S: u8 = 0x9;
const KEY_D: u8 = 0xA;
const KEY_F: u8 = 0xB;

const KEY_Z: u8 = 0xC;
const KEY_X: u8 = 0xD;
const KEY_C: u8 = 0xE;
const KEY_V: u8 = 0xF;


pub struct Keypad {
    pub last_pressed: Option<u8>,
    pub key_state: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            last_pressed: None,
            key_state: [false; 16]
        }
    }

    pub fn key_pressed(&mut self, key: Keycode) {
        match key {
            Keycode::Num1 => self.set_key(KEY_1),
            Keycode::Num2 => self.set_key(KEY_2),
            Keycode::Num3 => self.set_key(KEY_3),
            Keycode::Num4 => self.set_key(KEY_4),
            Keycode::Q => self.set_key(KEY_Q),
            Keycode::W => self.set_key(KEY_W),
            Keycode::E => self.set_key(KEY_E),
            Keycode::R => self.set_key(KEY_R),
            Keycode::A => self.set_key(KEY_A),
            Keycode::S => self.set_key(KEY_S),
            Keycode::D => self.set_key(KEY_D),
            Keycode::F => self.set_key(KEY_F),
            Keycode::Z => self.set_key(KEY_Z),
            Keycode::X => self.set_key(KEY_X),
            Keycode::C => self.set_key(KEY_C),
            Keycode::V => self.set_key(KEY_V),
            _ => {},
        }
    }

    pub fn key_released(&mut self, key: Keycode) {
        match key {
            Keycode::Num1 => self.release_key(KEY_1),
            Keycode::Num2 => self.release_key(KEY_2),
            Keycode::Num3 => self.release_key(KEY_3),
            Keycode::Num4 => self.release_key(KEY_4),
            Keycode::Q => self.release_key(KEY_Q),
            Keycode::W => self.release_key(KEY_W),
            Keycode::E => self.release_key(KEY_E),
            Keycode::R => self.release_key(KEY_R),
            Keycode::A => self.release_key(KEY_A),
            Keycode::S => self.release_key(KEY_S),
            Keycode::D => self.release_key(KEY_D),
            Keycode::F => self.release_key(KEY_F),
            Keycode::Z => self.release_key(KEY_Z),
            Keycode::X => self.release_key(KEY_X),
            Keycode::C => self.release_key(KEY_C),
            Keycode::V => self.release_key(KEY_V),
            _ => {},
        }
    }

    fn set_key(&mut self, key: u8) {
        self.key_state[key as usize] = true;
        self.last_pressed = Some(key);
    }

    fn release_key(&mut self, key: u8) {
        self.key_state[key as usize] = false;

        if self.last_pressed == Some(key) {
            self.last_pressed = None;
        }
    }
}