const NUM_REGS: usize = 16;
const NUM_STACK_FRAMES: usize = 16;
const MEM_SIZE: usize = 4 * 1024;
const PROGRAM_CODE_OFFSET: usize = 0x200;

pub struct CPU {
    v_regs: [u8; NUM_REGS],
    i_reg: u16,
    pc_reg: u16,
    delay_timer_reg: u8,
    sound_timer_reg: u8,

    memory: [u8; MEM_SIZE],
    stack: [u16; NUM_STACK_FRAMES],
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> CPU {
        let mut memory = [0; MEM_SIZE];

        for (i, byte) in rom.iter().enumerate() {
            memory[PROGRAM_CODE_OFFSET + i] = byte.clone();
        }

        CPU {
            v_regs: [0; 16],
            i_reg: 0,
            pc_reg: 0x200,
            delay_timer_reg: 0,
            sound_timer_reg: 0,
            memory: memory,
            stack: [0; NUM_STACK_FRAMES]
        }
    }
}