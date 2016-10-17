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

    pub fn tick(&mut self) {
        self.print_state();
        let opcode = self.fetch_opcode();
        self.pc_reg += 2;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                        0x0000 => self.op_00e0(opcode),
                        0x000E => self.op_00ee(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            0x1000 => self.op_1nnn(opcode),
            0x2000 => self.op_2nnn(opcode),
            0x3000 => self.op_3xnn(opcode),
            0x4000 => self.op_4xnn(opcode),
            0x5000 => self.op_5xy0(opcode),
            0x6000 => self.op_6xnn(opcode),
            0x7000 => self.op_7xnn(opcode),
            0x8000 => match opcode & 0x000F {
                        0x0000 => self.op_8xy0(opcode),
                        0x0001 => self.op_8xy1(opcode),
                        0x0002 => self.op_8xy2(opcode),
                        0x0003 => self.op_8xy3(opcode),
                        0x0004 => self.op_8xy4(opcode),
                        0x0005 => self.op_8xy5(opcode),
                        0x0006 => self.op_8xy6(opcode),
                        0x0007 => self.op_8xy7(opcode),
                        0x000E => self.op_8xye(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            0x9000 => self.op_9xy0(opcode),
            0xA000 => self.op_annn(opcode),
            0xB000 => self.op_bnnn(opcode),
            0xC000 => self.op_cxnn(opcode),
            0xD000 => self.op_dxyn(opcode),
            0xE000 => match opcode & 0x00FF {
                        0x009E => self.op_ex9e(opcode),
                        0x00A1 => self.op_exa1(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            0xF000 => match opcode & 0x00FF {
                        0x0007 => self.op_fx07(opcode),
                        0x000A => self.op_fx0a(opcode),
                        0x0015 => self.op_fx15(opcode),
                        0x0018 => self.op_fx18(opcode),
                        0x001E => self.op_fx1e(opcode),
                        0x0029 => self.op_fx29(opcode),
                        0x0033 => self.op_fx33(opcode),
                        0x0055 => self.op_fx55(opcode),
                        0x0065 => self.op_fx65(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            _       => println!("Unkown opcode: {:x}", opcode),
        };
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

    fn op_00e0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_00ee(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_1nnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_2nnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_3xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_4xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_5xy0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_6xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_7xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xy0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xy1(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }
    fn op_8xy2(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xy3(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xy4(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xy5(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xy6(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xy7(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_8xye(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_9xy0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_annn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_bnnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_cxnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_dxyn(&mut self, opcode: u16) {
       panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_ex9e(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_exa1(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx07(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx0a(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx15(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx18(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx1e(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx29(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx33(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx55(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    fn op_fx65(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }
}