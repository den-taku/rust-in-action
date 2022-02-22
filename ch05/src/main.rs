fn main() {
    let mut cpu = Cpu {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x21;
    mem[1] = 0x00;
    mem[2] = 0x21;
    mem[3] = 0x00;
    mem[4] = 0x00;
    mem[5] = 0x00;

    // for function
    let add_twice = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
    mem[0x100..0x106].copy_from_slice(&add_twice);

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

struct Cpu {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Cpu {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let op_minor = (opcode & 0x000F) as u8;

            let addr = opcode & 0x0FFF;
            let kk = (opcode & 0x00FF) as u8;

            match opcode {
                0x0000 => {
                    return;
                }
                0x00E0 => { /* CLEAR SCREEN */ }
                0x00EE => self.ret(),
                0x1000..=0x1FFF => self.jmp(addr),
                0x2000..=0x2FFF => self.call(addr),
                0x3000..=0x3FFF => self.se(x, kk),
                0x4000..=0x4FFF => self.sne(x, kk),
                0x5000..=0x5FFF => self.se(x, y),
                0x6000..=0x6FFF => self.ld(x, kk),
                0x7000..=0x7FFF => self.add(x, kk),
                0x8000..=0x8FFF => match op_minor {
                    0 => self.ld(x, self.registers[y as usize]),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => self.add_xy(x, y),
                    _ => todo!("opcode: {opcode:04x}"),
                },
                _ => todo!("opcode {opcode:04x}"),
            }
        }
    }

    /// (6xkk) LD sets the value `kk` into register `vx`
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }

    /// (7xkk) Add the value `kk` into register `vx`
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    fn se(&mut self, vx: u8, kk: u8) {
        if vx == kk {
            self.position_in_memory += 2;
        }
    }

    /// () SNE Store if Not Equal
    fn sne(&mut self, vx: u8, kk: u8) {
        if vx != kk {
            self.position_in_memory += 2;
        }
    }

    /// (1nnn) JUMP to `addr`
    fn jmp(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    /// (2nnn) CALL sub-rotine at `addr`
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp >= stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    /// (00ee) RET return from the current sub-routiine
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

    /// (7xkk)
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }
}
