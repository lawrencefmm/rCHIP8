use std::fs::File;
use std::io::{Read};
use rand::rngs::ThreadRng;
use rand::Rng;


const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;
const START_ADDRESS: u16 = 0x200;

pub struct Chip8 {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
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

impl Chip8 {
    pub fn new() -> Self {
        let mut cpu = Chip8 { 
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            stack: [0;16],
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

        cpu.memory[FONTSET_START_ADDRESS..(FONTSET_SIZE + FONTSET_START_ADDRESS)].copy_from_slice(&fontset[..FONTSET_SIZE]);

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
}