use emulator;

pub struct CPU {
    v_regs: [u8; emulator::NUM_REGS],
    i_reg: u16,
    pc_reg: u16,
    sp_reg: u8,
    delay_timer_reg: u8,
    sound_timer_reg: u8,

    memory: Vec<u8>,
    stack: [u16; emulator::NUM_STACK_FRAMES],
}

impl CPU {
    pub fn new(memory: Vec<u8>) -> CPU {
        CPU {
            v_regs: [0; 16],
            i_reg: 0,
            pc_reg: emulator::PROGRAM_OFFSET as u16,
            sp_reg: 0,
            delay_timer_reg: 0,
            sound_timer_reg: 0,
            memory: memory,
            stack: [0; emulator::NUM_STACK_FRAMES]
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
            println!("V{:X}: {:x}", i, reg)
        }

        println!("PC: {:x}", self.pc_reg);
        println!("IP: {:x}", self.i_reg);
        println!("Next opcode: {:#X}", self.fetch_opcode());
        println!("-----------------------\n");
    }
}