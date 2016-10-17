const NUM_KEYS: usize = 0xf;

pub struct Keypad {
    key: [u8; NUM_KEYS]
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            key: [0; NUM_KEYS]
        }
    }

    pub fn set_keys(&self) {

    }
}