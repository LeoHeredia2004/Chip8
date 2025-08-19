use rand::Rng;
use std::fs::File;
use std::io::Read;

const FONT_SET: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const MEMORY_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const N_REGISTERS: usize = 16;
const KEYPAD_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Chip8 {
    //Memory
    memory: [u8; MEMORY_SIZE],
    
    //Registers
    registers_v: [u8; N_REGISTERS],
    register_i: u16,
    pc: u16,

    //Stack
    stack: [u16; STACK_SIZE],
    stack_pointer: u16,

    //Display
    display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],

    //Keypad
    keypad: [bool;KEYPAD_SIZE],
}

impl Chip8 {
    pub fn new() -> Self{
        let mut chip8 = Chip8{
            memory: [0; MEMORY_SIZE],
            registers_v: [0; N_REGISTERS],
            register_i: 0,
            pc: 0x200, //INCIA EM 0X200 POIS OA ANTERIORES SÃO RESERVADOS PARA O SISTEMA
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            display: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keypad: [false; KEYPAD_SIZE],
        };

        for i in 0..80{
            chip8.memory[i] = FONT_SET[i];
        }

        chip8
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate(){
            let adress = i + 0x200;
            if adress < MEMORY_SIZE {
                self.memory[adress] = byte;
            }else{
                break;
            }
        }
    }

    pub fn emulate_cycle(&mut self){
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16);

        match opcode {
            //CLEAR SCREEN
            0x00E0 => {
                for y in 0..SCREEN_HEIGHT{
                    for x in 0..SCREEN_WIDTH{
                        self.display[y][x] = 0;
                    }
                }

                self.pc += 2;
            }
            //JUMP INSTRUCTIONS
            0x1000..=0x1FFF => {
                let adress = opcode & 0x0FFF;
                self.pc = adress;
            }
            //SET I
            0xA000..=0xAFFF => {
                let adress = opcode & 0x0FFF;
                self.register_i = opcode & 0x0FFF;
                self.pc += 2;
            }
            //SET REGISTER VX
            0x6000..=0x6FFF => {
                let register_index = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x0FF) as u8;
                self.registers_v[register_index] = value;
                self.pc += 2;
            }
            //ADD VALUE TO REGISTER VX
            0x7000..=0x7FFF => {
                let register_index = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x0FF) as u8;
                self.registers_v[register_index] = self.registers_v[register_index].wrapping_add(value);
                self.pc += 2;
            }
            //LOGICAL INSTRUCTIONS
            0x8000..=0x8FFF => {
                let x = (opcode & 0x0F00 >> 8) as usize;
                let y = (opcode & 0x00F0 >> 4) as usize;
                let n = opcode & 0x000F;
            
                match n {
                    //VX = VY
                    0x0 => {
                        self.registers_v[x] = self.registers_v[y];
                    }
                    //VX = VX OR VY
                    0x1 => {
                        self.registers_v[x] |= self.registers_v[y];
                    }
                    //VX = VX AND VY
                    0x2 => {
                        self.registers_v[x] &= self.registers_v[y];
                    }
                    //VX = VX XOR VY
                    0x3 => {
                        self.registers_v[x] ^= self.registers_v[y];
                    }
                    //VX = VX + VY
                    0x4 => {
                        let (result, carry) = self.registers_v[x].overflowing_add(self.registers_v[y]);
                        self.registers_v[x] = result;
                        self.registers_v[0xF] = if carry { 1 } else { 0 };
                    }
                    //VX = VX - VY
                    0x5 => {
                        let (result, borrow) = self.registers_v[x].overflowing_sub(self.registers_v[y]);
                        self.registers_v[x] = result;
                        self.registers_v[0xF] = if borrow { 0 } else { 1 };
                    }
                    //VX = VY SHIFT RIGHT 1
                    0x6 => {
                        let lsb = self.registers_v[x] & 0x1;
                        self.registers_v[x] >>= 1;
                        self.registers_v[0xF] = lsb;
                    }
                    0x7 => { // 0x8XY7: VX = VY - VX
                        let (result, borrow) = self.registers_v[y].overflowing_sub(self.registers_v[x]);
                        self.registers_v[x] = result;
                        self.registers_v[0xF] = if borrow { 0 } else { 1 };
                    }
                    //VX = VY SHIFT LEFT 1
                    0xE => {
                        let msb = (self.registers_v[x] & 0x80) >> 7;
                        self.registers_v[x] <<= 1;
                        self.registers_v[0xF] = msb;
                    }
                    _ => {
                        println!("Instrução: {:04X}", opcode);
                    }
                }
                self.pc += 2;
            
            }
            //JUMP IF VX == KK
            0x3000..=0x3FFF => {
                let x = (opcode & 0x0F00 >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                if self.registers_v[x] == kk {
                    self.pc += 4;
                }else {
                    self.pc += 2;
                }
            }
            //JUMP IF VX != KK
            0x4000..=0x4FFF => {
                let x = (opcode & 0x0F00 >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                if self.registers_v[x] != kk {
                    self.pc += 4;
                }else {
                    self.pc += 2;
                }
            }
            //JUMP IF VX == VY
            0x5000..=0x5FFF => {
                let x = (opcode & 0x0F00 >> 8) as usize;
                let y = (opcode & 0x00F0 >> 4) as usize;
                if self.registers_v[x] == self.registers_v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            //JUMP IF VX != VY
            0x6000..=0x6FFF => {
                let x = (opcode & 0x0F00 >> 8) as usize;
                let y = (opcode & 0x00F0 >> 4) as usize;
                if self.registers_v[x] != self.registers_v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            //JUMP
            0x2000..=0x2FFF => {
                let adress = opcode & 0x0FFF;
                self.stack[self.stack_pointer as usize] = self.pc;
                self.stack_pointer += 1;
                self.pc = adress;
            }
            //RETURN
            0x00EE => {
                self.stack_pointer -= 1;
                self.pc = self.stack[self.stack_pointer as usize];
                self.pc += 2;
            }

            _ => {
                println!("Opcode não existe!");
            }
        }
    }

}

fn main() {
    let mut chip8 =Chip8::new();

    let rom_path = "C:/Users/leohe/OneDrive/Documentos/chip8/chip8_emulator/roms/Pong (alt).ch8";

    match File::open(rom_path){
        Ok(mut file) => {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            chip8.load_rom(&data);
            println!("ROM carregada!");
        }
        Err(e) => {
            println!("Erro ao abrir o arquivo ROM: {}", e);
            return;
        }
    }

    loop {
        chip8.emulate_cycle();
    }
}
