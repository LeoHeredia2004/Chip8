use rand::Rng;

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
    stack: [u16, STACK_SIZE],
    stack_pointer: u16,

    //Display
    display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],

    //Keypad
    keypad: [bool;KEYPAD_SIZE],
}

impl Chip8 {
    pub fn new() -> self{
        let mut chip8 = Chip8{
            memory: [0, MEMORY_SIZE],
            registers_v: [0, N_REGISTERS],
            register_i: 0,
            pc: 0x200, //INCIA EM 0X200 POIS OA ANTERIORES SÃO RESERVADOS PARA O SISTEMA
            stack: [0, STACK_SIZE],
            stack_pointer: 0,
            display: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keypad: [false; KEYPAD_SIZE],
        }

        for i in 0..80{
            chip8.memory[i] = FONT_SET[i];
        }

        chip8
    }

}

fn main() {
    println!("CHIP8");
}
