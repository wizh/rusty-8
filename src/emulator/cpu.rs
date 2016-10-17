const NUM_REGS: usize = 16;
const NUM_STACK_FRAMES: usize = 16;
const MEM_SIZE: usize = 4 * 1024;
const PROGRAM_CODE_OFFSET: usize = 0x200;

pub struct CPU {
    v_regs: [u8; NUM_REGS],
    i_reg: u16,
    pc_reg: u16,
    sp_reg: u8,
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
            pc_reg: PROGRAM_CODE_OFFSET,
            sp_reg: 0,
            delay_timer_reg: 0,
            sound_timer_reg: 0,
            memory: memory,
            stack: [0; NUM_STACK_FRAMES]
        }
    }

    pub fn tick(&mut self){
        self.print_state();
        let opcode = self.fetch_opcode();
        self.pc_reg += 2;

        // match opcode
    }

    fn fetch_opcode(&self) -> u16 {
        (self.memory[self.pc_reg as usize] as u16) << 8 |
        self.memory[(self.pc_reg as usize) + 1] as u16
    }

    fn print_state(&self) {
        println!("-----------------------");
        for (i, reg) in self.v_regs.iter().enumerate() {
            println!("Reg {:x}: {:x}", i, reg)
        }
        println!("PC: {:x}", self.pc_reg);
        println!("IP: {:x}", self.i_reg);
        println!("Next opcode: {:#X}", self.fetch_opcode());
        println!("-----------------------\n");
    }
}