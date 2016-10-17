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

    // Clears the screen.
    fn op_00e0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Returns from a subroutine.
    fn op_00ee(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Jumps to address NNN.
    fn op_1nnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Calls subroutine at NNN.
    fn op_2nnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Skips the next instruction if VX equals NN.
    fn op_3xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Skips the next instruction if VX doesn't equal NN.
    fn op_4xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Skips the next instruction if VX equals VY.
    fn op_5xy0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to NN.
    fn op_6xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Adds NN to VX.
    fn op_7xnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to VX OR VY.
    fn op_8xy0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to VX OR VY.
    fn op_8xy1(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to VX AND VY.
    fn op_8xy2(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to VX XOR VY.
    fn op_8xy3(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there
    // isn't.
    fn op_8xy4(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1
    // when there isn't.
    fn op_8xy5(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Shifts VX right by one. VF is set to the value of the least significant
    // bit of VX before the shift.
    fn op_8xy6(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1
    // when there isn't.
    fn op_8xy7(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Shifts VX left by one. VF is set to the value of the most significant
    // bit of VX before the shift.
    fn op_8xye(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Skips the next instruction if VX doesn't equal VY.
    fn op_9xy0(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets I to the address NNN.
    fn op_annn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Jumps to the address NNN plus V0.
    fn op_bnnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to the result of a bitwise AND on a random number and NN.
    fn op_cxnn(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and
    // a height of N pixels. Each row of 8 pixels is read as bit-coded
    // starting from memory location I; I value doesn’t change after the
    // execution of this instruction. VF is set to 1 if any screen pixels are
    // flipped from set to unset when the sprite is drawn, and to 0 if that
    // doesn’t happen.
    fn op_dxyn(&mut self, opcode: u16) {
       panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Skips the next instruction if the key stored in VX is pressed.
    fn op_ex9e(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Skips the next instruction if the key stored in VX isn't pressed.
    fn op_exa1(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets VX to the value of the delay timer.
    fn op_fx07(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // A key press is awaited, and then stored in VX.
    fn op_fx0a(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets the delay timer to VX.
    fn op_fx15(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets the sound timer to VX.
    fn op_fx18(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Adds VX to I.
    fn op_fx1e(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Sets I to the location of the sprite for the character in VX.
    // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    fn op_fx29(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Stores the binary-coded decimal representation of VX, with the most
    // significant of three digits at the address in I, the middle digit at I
    // plus 1, and the least significant digit at I plus 2.
    fn op_fx33(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Stores V0 to VX (including VX) in memory starting at address I.
    fn op_fx55(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }

    // Fills V0 to VX (including VX) with values from memory starting at address I.
    fn op_fx65(&mut self, opcode: u16) {
        panic!("Uninplemented opcode: {:x}", opcode);
    }
}