use std::fs::File;
use std::io::{Read};
use rand::rngs::ThreadRng;
use rand::Rng;


const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;
const START_ADDRESS: u16 = 0x200;
const VIDEO_WIDTH: u8 = 64;
const VIDEO_HEIGHT: u8 = 32;

pub struct Chip8 {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub keypad: [u16; 16],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub video: [u32; 64*32],
    pub opcode: u16,
    pub rand_gen: ThreadRng,
    pub rand_byte: u8
}

fn get_vx(opcode: u16) -> usize {
    (opcode as usize & 0x0F00) >> 8
}

fn get_vy(opcode: u16) -> usize {
    (opcode as usize & 0x00F0) >> 4
}

fn get_byte(opcode: u16) -> u8 {
    opcode as u8 & 0x00FF
}

#[allow(non_snake_case)]
impl Chip8 {
    pub fn new() -> Self {
        let mut cpu = Chip8 { 
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            stack: [0;16],
            keypad: [0;16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            video: [0; 64*32],
            opcode: 0,
            rand_gen: rand::thread_rng(),
            rand_byte: 0
        };
        

        let fontset: [u8; FONTSET_SIZE] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        cpu.memory[FONTSET_START_ADDRESS..(
            FONTSET_SIZE + FONTSET_START_ADDRESS)
        ].copy_from_slice(&fontset[..FONTSET_SIZE]);

        cpu.rand_byte = cpu.rand_gen.r#gen();

        cpu

    }

    pub fn load_rom(&mut self, filename: &String) -> std::io::Result<Vec<u8>>{
        let mut file = File::open(filename)?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        for (i, byte) in buffer.iter().enumerate() {
            self.memory[(self.pc as usize) + i] = *byte;
        }

        Ok(buffer)
    }

    

    pub fn OP_00E0(&mut self) {
        self.video.fill(0);
    }

	pub fn OP_NULL(&mut self) {

    }

	pub fn OP_00EE(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

	pub fn OP_1NNN(&mut self) {
        self.pc = self.opcode & 0x0FFF;
    }

	pub fn OP_2NNN(&mut self) {
        let address = self.opcode & 0x0FFF;

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

	pub fn OP_3XKK(&mut self) {
        let Vx = get_vx(self.opcode);
        let byte = get_byte(self.opcode);

        if self.registers[Vx] == byte {
            self.pc += 2;
        }
    }

	pub fn OP_4XKK(&mut self) {
        let Vx = get_vx(self.opcode);
        let byte = get_byte(self.opcode);

        if self.registers[Vx] != byte {
            self.pc += 2;
        }
    }

	pub fn OP_5XY0(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        if self.registers[Vx] == self.registers[Vy] {
            self.pc += 2;
        }
    }

	pub fn OP_6XKK(&mut self) {
        let Vx = get_vx(self.opcode);
        let byte = get_byte(self.opcode);

        self.registers[Vx] = byte;
    }

	pub fn OP_7XKK(&mut self) {
        let Vx = get_vx(self.opcode);
        let byte = get_byte(self.opcode);

        self.registers[Vx] += byte;
    }

	pub fn OP_8XY0(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        self.registers[Vx] = self.registers[Vy];
    }

	pub fn OP_8XY1(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        self.registers[Vx] |= self.registers[Vy];
    }

	pub fn OP_8XY2(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        self.registers[Vx] &= self.registers[Vy];
    }

	pub fn OP_8XY3(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        self.registers[Vx] ^= self.registers[Vy];
    }

	pub fn OP_8XY4(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        let sum = self.registers[Vx] + self.registers[Vy];
        
        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[Vx] = sum & 0xFF;
    }

	pub fn OP_8XY5(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        
        if self.registers[Vx] > self.registers[Vy] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[Vx] -= self.registers[Vy];
    }

	pub fn OP_8XY6(&mut self) {
        let Vx = get_vx(self.opcode);
        self.registers[0xF] = self.registers[Vx] & 0x1;
        self.registers[Vx] >>= 1;
    }

	pub fn OP_8XY7(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);

        
        if self.registers[Vy] > self.registers[Vx] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[Vx] = self.registers[Vy] - self.registers[Vx];
    }

	pub fn OP_8XYE(&mut self) {
        let Vx = get_vx(self.opcode);
        self.registers[0xF] = (self.registers[Vx] & 0x80) >> 7;
        self.registers[Vx] <<= 1;
    }

	pub fn OP_9XY0(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);
        if Vx == Vy {
            self.pc += 2
        }
    }

	pub fn OP_ANNN(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.index = address;
    }

	pub fn OP_BNNN(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.pc = self.registers[0] as u16 + address;
    }

	pub fn OP_CXKK(&mut self) {
        let Vx = get_vx(self.opcode);
        let byte = get_byte(self.opcode);

        self.registers[Vx] = self.rand_gen.r#gen::<u8>() & byte;
    }

	pub fn OP_DXYN(&mut self) {
        let Vx = get_vx(self.opcode);
        let Vy = get_vy(self.opcode);
        let height = self.opcode as u8 & 0xF;

        let xPos = self.registers[Vx] % VIDEO_WIDTH;
        let yPos = self.registers[Vy] % VIDEO_HEIGHT;

        self.registers[0xF] = 0;

        for row in 0..height {
            let spriteByte = self.memory[(self.index as u8  + row) as usize];

            for col in 0u8..8u8 {
                let spritePixel = spriteByte & (0x80u8 >> col);

                let screenPixel = ((yPos + row) * VIDEO_WIDTH + (xPos + col)) as usize;

                if spritePixel == 1 {
                    if self.video[screenPixel] == 0xFFFFFFFF {
                        self.registers[0xF] = 1;
                    }
                    self.video[screenPixel] ^= 0xFFFFFFFF;
                }
            }
        }
    }

	pub fn OP_EX9E(&mut self) {
        let Vx = get_vx(self.opcode);
        let key = self.registers[Vx] as usize;

        if self.keypad[key] != 0 {
            self.pc += 2;
        }
    }

	pub fn OP_EXA1(&mut self) {
        let Vx = get_vx(self.opcode);
        let key = self.registers[Vx] as usize;

        if self.keypad[key] == 0 {
            self.pc += 2;
        }
    }

	pub fn OP_FX07(&mut self) {
        let Vx = get_vx(self.opcode);
        self.registers[Vx] = self.delay_timer;
    }

	pub fn OP_FX0A(&mut self) {
        let Vx = get_vx(self.opcode);

        for i in 0..16 {
            if self.keypad[i] != 0 {
                self.registers[Vx] = i as u8;
                return;
            }
        }

        self.pc -= 2;
    }

	pub fn OP_FX15(&mut self) {
        let Vx = get_vx(self.opcode);
        self.delay_timer = self.registers[Vx];
    }

	pub fn OP_FX18(&mut self) {
        let Vx = get_vx(self.opcode);
        self.sound_timer = self.registers[Vx];
    }

	pub fn OP_FX1E(&mut self) {
        let Vx = get_vx(self.opcode);
        self.index += self.registers[Vx] as u16;
    }

	pub fn OP_FX29(&mut self) {
        let Vx = get_vx(self.opcode);
        let digit = self.registers[Vx];

        self.index = FONTSET_START_ADDRESS as u16 + (5 * digit as u16);
    }

	pub fn OP_FX33(&mut self) {
        let Vx = get_vx(self.opcode);
        let mut value = self.registers[Vx];

        self.memory[(self.index + 2) as usize] = value % 10;
        value /= 10;

        self.memory[(self.index + 1) as usize] = value % 10;
        value /= 10;

        self.memory[self.index as usize] = value % 10;
    }

	pub fn OP_FX55(&mut self) {
        let Vx = get_vx(self.opcode);

        for i in 0..=Vx {
            self.memory[self.index as usize + i] = self.registers[i];
        }
    }

	pub fn OP_FX65(&mut self) {
        let Vx = get_vx(self.opcode);

        for i in 0..=Vx {
            self.registers[i] = self.memory[self.index as usize + i]
        }

    }

    pub fn Cycle(&mut self) {
        self.opcode = ((self.memory[self.pc as usize] << 8) |
                       self.memory[self.pc as usize + 1]) as u16;
    
        self.pc += 2;

        
    }

}