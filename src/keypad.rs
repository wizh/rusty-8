use emulator;

pub struct Keypad {
    key: [u8; emulator::NUM_KEYS]
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            key: [0; emulator::NUM_KEYS]
        }
    }

    pub fn set_keys(&self) {

    }
}