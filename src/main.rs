use rand::Rng;

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


fn main() {
    println!("CHIP8");
}
