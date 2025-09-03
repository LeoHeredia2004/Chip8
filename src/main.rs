const UPSCALE: usize = 10;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use std::thread;
use rodio::{OutputStream, Sink, source::{SineWave, Source}};


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

//BIG FONT SET PARA AS FONTES MAIORES DO XO-CHIP
const BIG_FONT_SET: [u8; 160] = [
    0x3C, 0x7E, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0x7E, 0x3C, // 0
    0x18, 0x38, 0x58, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, // 1
    0x3E, 0x7F, 0xC3, 0x06, 0x0C, 0x18, 0x30, 0x60, 0xFF, 0xFF, // 2
    0x3C, 0x7E, 0xC3, 0x03, 0x0E, 0x0E, 0x03, 0xC3, 0x7E, 0x3C, // 3
    0x06, 0x0E, 0x1E, 0x36, 0x66, 0xC6, 0xFF, 0xFF, 0x06, 0x06, // 4
    0xFF, 0xFF, 0xC0, 0xC0, 0xFC, 0xFE, 0x03, 0xC3, 0x7E, 0x3C, // 5
    0x3E, 0x7C, 0xC0, 0xC0, 0xFC, 0xFE, 0xC3, 0xC3, 0x7E, 0x3C, // 6
    0xFF, 0xFF, 0x03, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x30, // 7
    0x3C, 0x7E, 0xC3, 0xC3, 0x7E, 0x7E, 0xC3, 0xC3, 0x7E, 0x3C, // 8
    0x3C, 0x7E, 0xC3, 0xC3, 0x7F, 0x3F, 0x03, 0x03, 0x3E, 0x7C, // 9
    0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, // A
    0xFC, 0xFE, 0xC3, 0xC3, 0xFC, 0xFE, 0xC3, 0xC3, 0xFE, 0xFC, // B
    0x3E, 0x7C, 0xC0, 0xC0, 0xC0, 0xC0, 0xC0, 0xC0, 0x7C, 0x3E, // C
    0xF8, 0xFC, 0xCE, 0xC6, 0xC6, 0xC6, 0xC6, 0xCE, 0xFC, 0xF8, // D
    0xFF, 0xFF, 0xC0, 0xC0, 0xF8, 0xF8, 0xC0, 0xC0, 0xFF, 0xFF, // E
    0xFF, 0xFF, 0xC0, 0xC0, 0xF8, 0xF8, 0xC0, 0xC0, 0xC0, 0xC0  // F
];

const KEY_MAP: [minifb::Key; 16] = [
    minifb::Key::X,     // 0
    minifb::Key::Key1,  // 1
    minifb::Key::Key2,  // 2
    minifb::Key::Key3,  // 3
    minifb::Key::Q,     // 4
    minifb::Key::W,     // 5
    minifb::Key::E,     // 6
    minifb::Key::A,     // 7
    minifb::Key::S,     // 8
    minifb::Key::D,     // 9
    minifb::Key::Z,     // A
    minifb::Key::C,     // B
    minifb::Key::Key4,  // C
    minifb::Key::R,     // D
    minifb::Key::F,     // E
    minifb::Key::V,     // F
];

const MEMORY_SIZE: usize = 4096;
const N_REGISTERS: usize = 16;
const KEYPAD_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Chip8 {
    screen_width: usize,
    screen_height: usize,
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
    display: [[u8; 128]; 64],

    //Keypad
    keypad: [bool;KEYPAD_SIZE],

    draw_flag: bool,

    dt: u8,
    st: u8,

    high_res_mode: bool,

    is_running: bool,
}

impl Chip8 {
    pub fn new() -> Self{
        let mut chip8 = Chip8{
            screen_width: 64,
            screen_height: 32,
            memory: [0; MEMORY_SIZE],
            registers_v: [0; N_REGISTERS],
            register_i: 0,
            pc: 0x200, //INCIA EM 0X200 POIS OA ANTERIORES SÃO RESERVADOS PARA O SISTEMA
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            display: [[0; 128]; 64],
            keypad: [false; KEYPAD_SIZE],
            draw_flag: false,
            dt: 0,
            st: 0,
            high_res_mode: false,
            is_running: true,
        };

        for i in 0..80{
            chip8.memory[i] = FONT_SET[i];
        }

        for i in 0..160{
            chip8.memory[80 + i] = BIG_FONT_SET[i];
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
            // JUMP V0 + addr
            0xB000..=0xBFFF => {
                let addr = opcode & 0x0FFF;
                self.pc = addr + self.registers_v[0] as u16;
            }
            // RND Vx, byte
            0xC000..=0xCFFF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                self.registers_v[x] = rng.r#gen::<u8>() & nn;
                self.pc += 2;
            }
            //SYSTEM INSTRUCTIONS
            0x0000..=0x0FFF => {
                match opcode & 0x00FF{
                    //CLEAR SCREEN
                    0xE0 => {
                        for y in 0..self.screen_height{
                            for x in 0..self.screen_width{
                                self.display[y][x] = 0;
                            }
                        }

                        self.pc += 2;
                    }
                    //RETURN
                    0xEE => {
                        if self.stack_pointer > 0 {
                            self.stack_pointer -= 1;
                            self.pc = self.stack[self.stack_pointer as usize];
                            self.pc += 2;
                        } else {
                            println!("Stack underflow em RETURN! Ignorando retorno.");
                            self.is_running = false;
                        }
                    }
                    //ACTIVATE HIGH RESOLUTION MODE
                    0xFF => {
                        self.high_res_mode = true;
                        self.screen_width = 128;
                        self.screen_height = 64;
                        self.pc += 2;
                    }
                    //DEACTIVATE HIGH RESOLUTION MODE
                    0xFE => {
                        self.high_res_mode = false;
                        self.screen_width = 64;
                        self.screen_height = 32;
                        self.pc += 2;
                    }
                    //ROLL SCREEN DOWN
                    0xC0..=0xCF => {
                        let n = (opcode & 0x000F) as usize;
                        
                        for y in (n..self.screen_height).rev() {
                            for x in 0..self.screen_width {
                                self.display[y][x] = self.display[y - n][x];
                            }
                        }

                        for y in 0..n {
                            for x in 0..self.screen_width {
                                self.display[y][x] = 0;
                            }
                        }

                        self.draw_flag = true;
                        self.pc += 2;
                    }
                    //ROLL SCREEN RIGHT
                    0xFB => {
                        for y in 0..self.screen_height {
                            for x in (4..self.screen_width).rev(){
                                self.display[y][x] = self.display[y][x - 4];
                            }

                            for x in 0..4{
                                self.display[y][x] = 0;
                            }
                        }

                        self.draw_flag = true;
                        self.pc += 2;
                    }
                    //ROLL SCREEN LEFT
                    0xFC => {
                        for y in 0..self.screen_height {
                            for x in (0..(self.screen_width - 4)){
                                self.display[y][x] = self.display[y][x + 4];
                            }

                            for x in (self.screen_width - 4)..self.screen_width{
                                self.display[y][x] = 0;
                            }
                        }

                        self.draw_flag = true;
                        self.pc += 2;
                    }
                    //EXIT
                    0xFD => {
                        self.is_running = false;
                    }
                    _ => {
                        self.pc += 2;
                    }
                }
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
                let value = (opcode & 0x00FF) as u8;
                self.registers_v[register_index] = value;
                self.pc += 2;
            }
            //ADD VALUE TO REGISTER VX
            0x7000..=0x7FFF => {
                let register_index = ((opcode & 0x0F00) >> 8) as usize;
                let value = (opcode & 0x00FF) as u8;
                self.registers_v[register_index] = self.registers_v[register_index].wrapping_add(value);
                self.pc += 2;
            }
            //LOGICAL INSTRUCTIONS
            0x8000..=0x8FFF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
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
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                if self.registers_v[x] == kk {
                    self.pc += 4;
                }else {
                    self.pc += 2;
                }
            }
            //JUMP IF VX != KK
            0x4000..=0x4FFF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let kk = (opcode & 0x00FF) as u8;
                if self.registers_v[x] != kk {
                    self.pc += 4;
                }else {
                    self.pc += 2;
                }
            }
            //JUMP IF VX == VY
            0x5000..=0x5FFF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.registers_v[x] == self.registers_v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            //JUMP IF VX != VY
            0x9000..=0x9FFF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.registers_v[x] != self.registers_v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            //JUMP
            0x2000..=0x2FFF => {
                let adress = opcode & 0x0FFF;
                if (self.stack_pointer as usize) < STACK_SIZE {
                    self.stack[self.stack_pointer as usize] = self.pc;
                    self.stack_pointer += 1;
                    self.pc = adress;
                } else {
                    println!("Stack overflow em CALL! Ignorando chamada.");
                    self.pc += 2;
                }
            }
            //DRAW
            0xD000..=0xDFFF => {
                let x = self.registers_v[((opcode & 0x0F00) >> 8) as usize];
                let y = self.registers_v[((opcode & 0x00F0) >> 4) as usize];
                let n = (opcode & 0x000F) as usize;

                let mut collision = false;

                if self.high_res_mode == false {
                    for row in 0..n{
                        let sprite_byte = self.memory[(self.register_i + row as u16) as usize];

                        for col in 0..8{
                            let sprite_pixel = (sprite_byte >> (7 - col)) & 0x01;

                            if sprite_pixel == 1 {
                                let display_x = (x as usize + col) as usize % self.screen_width;
                                let display_y = (y as usize + row) as usize % self.screen_height;
                                
                                if self.display[display_y][display_x] == 1 {
                                    collision = true;
                                }
                                
                                self.display[display_y][display_x] ^= 1;
                            }
                        }


                    }
                }else{
                    if n == 0{
                        self.screen_width = 128;
                        self.screen_height = 64;
                        for row in 0..16{
                        let sprite_byte1 = self.memory[(self.register_i + (row as u16) * 2) as usize];
                        let sprite_byte2 = self.memory[(self.register_i + (row as u16) * 2 + 1) as usize];

                        for col in 0..8{
                            let sprite_pixel = (sprite_byte1 >> (7 - col)) & 0x01;

                            if sprite_pixel == 1 {
                                let display_x = (x as usize + col) as usize % self.screen_width;
                                let display_y = (y as usize + row) as usize % self.screen_height;
                                
                                if self.display[display_y][display_x] == 1 {
                                    collision = true;
                                }
                                
                                self.display[display_y][display_x] ^= 1;
                            }
                        }


                        for col in 0..8{
                            let sprite_pixel = (sprite_byte2 >> (7 - col)) & 0x01;

                            if sprite_pixel == 1 {
                                let display_x = (x as usize + col + 8) as usize % self.screen_width;
                                let display_y = (y as usize + row) as usize % self.screen_height;
                                
                                if self.display[display_y][display_x] == 1 {
                                    collision = true;
                                }
                                
                                self.display[display_y][display_x] ^= 1;
                            }
                        }

                    }
                    }else{
                        self.screen_width = 128;
                        self.screen_height = 64;
                        for row in 0..n{
                        let sprite_byte = self.memory[(self.register_i + row as u16) as usize];

                        for col in 0..8{
                            let sprite_pixel = (sprite_byte >> (7 - col)) & 0x01;

                            if sprite_pixel == 1 {
                                let display_x = (x as usize + col) as usize % self.screen_width;
                                let display_y = (y as usize + row) as usize % self.screen_height;
                                
                                if self.display[display_y][display_x] == 1 {
                                    collision = true;
                                }
                                
                                self.display[display_y][display_x] ^= 1;
                            }
                        }


                    }
                    }

                }

                self.registers_v[0xF] = if collision { 1 } else { 0 };
                self.draw_flag = true;
                self.pc += 2;
            }
            //JUMP IF PRESSED
            _ if (opcode & 0xF0FF) == 0xE09E => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if self.keypad[self.registers_v[x] as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            //JUMP IF NOT PRESSED
            _ if (opcode & 0xF0FF) == 0xE0A1 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if !self.keypad[self.registers_v[x] as usize]{
                    self.pc += 4;
                }else {
                    self.pc += 2;
                }
            }
            //WAIT FOR KEY
            _ if (opcode & 0xF0FF) == 0xF00A => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let mut key_pressed = false;
                
                for i in 0..16 {
                    if self.keypad[i] {
                        self.registers_v[x] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }
                
                if !key_pressed {
                    self.pc -= 2;
                } else {
                    self.pc += 2;
                }
            }
            //SET DT
            _ if (opcode & 0xF0FF) == 0xF015 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.dt = self.registers_v[x];
                self.pc += 2;
            }
            //SET ST
            _ if (opcode & 0xF0FF) == 0xF018 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.st = self.registers_v[x];
                self.pc += 2;
            }
            //SET VX = DT
            _ if (opcode & 0xF0FF) == 0xF007 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.registers_v[x] = self.dt;
                self.pc += 2;
            }
            //ADD I + VX
            _ if (opcode & 0xF0FF) == 0xF01E => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.register_i = self.register_i.wrapping_add(self.registers_v[x] as u16);
                self.pc += 2;
            }
            //SET I = SPRITE(VX)
            _ if (opcode & 0xF0FF) == 0xF029 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.register_i = (self.registers_v[x] as u16) * 5;
                self.pc += 2;
            }
            //STORE BCD
            _ if (opcode & 0xF0FF) == 0xF033 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let vx = self.registers_v[x];
                self.memory[self.register_i as usize] = vx / 100;
                self.memory[self.register_i as usize + 1] = (vx % 100) / 10;
                self.memory[self.register_i as usize + 2] = vx % 10;
                self.pc += 2;
            }
            //STORE REGISTERS
            _ if (opcode & 0xF0FF) == 0xF055 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                for i in 0..=x {
                    self.memory[self.register_i as usize + i] = self.registers_v[i];
                }
                self.pc += 2;
            }
            //LOAD REGISTERS
            _ if (opcode & 0xF0FF) == 0xF065 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                for i in 0..=x {
                    self.registers_v[i] = self.memory[self.register_i as usize + i];
                }
                self.pc += 2;
            }
            _ if (opcode & 0xF0FF) == 0xF030 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.register_i = 80 + (self.registers_v[x] as u16) * 10;
                self.pc +=2;
            }
            _ => {
                println!("Opcode não existe!: {:04X}", opcode);
            }
        }
    }

}

fn main() {
    let mut chip8 =Chip8::new();
    let mut cycle_counter = 0;

    const CPU_SPEED_HZ: u64 = 500;
    const TIMER_SPEED_HZ: u64 = 60;
    const CYCLES_PER_FRAME: u64 = CPU_SPEED_HZ / TIMER_SPEED_HZ;

    let frame_duration = Duration::from_millis(1000 / TIMER_SPEED_HZ);

    let rom_path = "C:/Users/leohe/OneDrive/Documentos/chip8/chip8_emulator/Chip8-Database/SuperChip8-Games/Snow Daze (unknown author)(2015).sc8";

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

    let mut window = Window::new(
        "Chip8 Emulator",
        chip8.screen_width * UPSCALE,
        chip8.screen_height * UPSCALE,
        WindowOptions::default(),
    ).unwrap();

    let mut buffer: Vec<u32> = vec![0; (chip8.screen_width * UPSCALE) * (chip8.screen_height * UPSCALE)];


    let mut prev_width = chip8.screen_width;
    let mut prev_height = chip8.screen_height;

    let(_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(1.5)).amplify(0.20).repeat_infinite();

    sink.append(source);
    sink.pause();
    
    while window.is_open() && !window.is_key_down(Key::Escape) && chip8.is_running {
        let frame_start = Instant::now();

        for i in 0..16 {
            if window.is_key_down(KEY_MAP[i]) {
                chip8.keypad[i] = true;
            } else {
                chip8.keypad[i] = false;
            }
        }

        for _ in 0..CYCLES_PER_FRAME {
            chip8.emulate_cycle();
        }

        if chip8.dt > 0 {
            chip8.dt -= 1;
        }
        if chip8.st > 0 {
            sink.play();
            chip8.st -= 1;
        }else{
            sink.pause();
        }

        chip8.emulate_cycle();

        if chip8.screen_width != prev_width || chip8.screen_height != prev_height {
            window = Window::new(
                "Chip8 Emulator",
                chip8.screen_width * UPSCALE,
                chip8.screen_height * UPSCALE,
                WindowOptions::default(),
            ).unwrap();
            buffer = vec![0; (chip8.screen_width * UPSCALE) * (chip8.screen_height * UPSCALE)];
            prev_width = chip8.screen_width;
            prev_height = chip8.screen_height;
        }

        if chip8.draw_flag {
            for y in 0..chip8.screen_height {
                for x in 0..chip8.screen_width {
                    let color = if chip8.display[y][x] == 1 { 0xFFFFFF } else { 0x0 };
                    for dy in 0..UPSCALE {
                        for dx in 0..UPSCALE {
                            let up_x = x * UPSCALE + dx;
                            let up_y = y * UPSCALE + dy;
                            let index = up_x + up_y * (chip8.screen_width * UPSCALE);
                            buffer[index] = color;
                        }
                    }
                }
            }
            window.update_with_buffer(&buffer, chip8.screen_width * UPSCALE, chip8.screen_height * UPSCALE).unwrap();
            chip8.draw_flag = false;
        }else{
            window.update();
        }

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }


    }
}
