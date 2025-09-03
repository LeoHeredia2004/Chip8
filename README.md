# CHIP8
### ENGLISH VERSION BELOW

## Emulador de Chip-8 em Rust
Um emulador de Chip-8, Super Chip-8 escrito em Rust, utilizando minifb para a criação da janela e renderização, e rodio para a reprodução de áudio. Este projeto foi criado com o objetivo de aprender mais sobre emulação e a linguagem Rust.

O que é o Chip-8?
Chip-8 é uma linguagem de programação interpretada, desenvolvida por Joseph Weisbecker. Foi inicialmente usada no microcomputador COSMAC VIP de 8 bits em meados da década de 1970. O Chip-8 é o que hoje chamaríamos de "fantasy console" ou "máquina virtual". Programas Chip-8 rodam em uma máquina virtual Chip-8. Nas décadas de 1970 e 1980, jogos para esta máquina virtual foram distribuídos em revistas de informática e como "type-in programs", onde o usuário digitava o código-fonte.

Atualmente, o Chip-8 é uma ótima maneira de começar a aprender sobre o desenvolvimento de emuladores, devido à sua simplicidade.

## Funcionalidades
### Emulação Completa de Chip-8: Suporte para todas as 35 opcodes originais.

### Suporte a Super Chip-8 (SCHIP):

Modo de alta resolução (128x64 pixels).

Novas opcodes para rolagem de tela (para a direita, esquerda e para baixo).

Sprites maiores (16x16).

### Áudio: Emissão de bipes sonoros através da biblioteca rodio.

### Controles Mapeados: Mapeamento de teclado para uma experiência de jogo mais confortável em teclados modernos.

Instruções para execução e demonstração abaixo do texto em inglês.


## Chip-8 Emulator in Rust
A Chip-8, Super Chip-8, and XO-CHIP emulator written in Rust, using minifb for window creation and rendering, and rodio for audio playback. This project was created with the goal of learning more about emulation and the Rust language.

## What is Chip-8?
Chip-8 is an interpreted programming language, developed by Joseph Weisbecker. It was initially used on the 8-bit COSMAC VIP microcomputer in the mid-1970s. Chip-8 is what we would today call a "fantasy console" or "virtual machine." Chip-8 programs run on a Chip-8 virtual machine. In the 1970s and 1980s, games for this virtual machine were distributed in computer magazines and as "type-in programs," where the user would type in the source code.

Today, Chip-8 is a great way to start learning about emulator development due to its simplicity.

## Features
### Full Chip-8 Emulation: Support for all 35 original opcodes.

### Super Chip-8 (SCHIP) Support:

High-resolution mode (128x64 pixels).

New opcodes for screen scrolling (right, left, and down).

Larger sprites (16x16).

### Audio: Sound beep emission through the rodio library.

### Mapped Controls: Keyboard mapping for a more comfortable gaming experience on modern keyboards.


## Como Executar / How to execute:
Para compilar e executar o emulador, você precisará ter o Rust e o Cargo instalados.
You need to have Rust and cargo installed.

### Clone o repositório / Clone repository:
git clone https://github.com/seu-usuario/seu-repositorio.git
cd seu-repositorio

### Altere o caminho da ROM / Change ROM path:
No arquivo src/main.rs, localize a seguinte linha e altere o caminho para a ROM que você deseja carregar:
On src/main.rs, find the following line and change it for the desired ROM:

let rom_path = "C:/Users/leohe/OneDrive/Documentos/chip8/chip8_emulator/Chip8-Database/SuperChip8-Games/Snow Daze (unknown author)(2015).sc8";

### Compile e execute o projeto / compile and execute project:
cargo run --release


O emulador iniciará e carregará a ROM especificada.
Emulator will start with the specified ROM.
(Eu não sou o criador de nenhuma das roms, elas foram obtidas através do internet archive)
(I´m not the creator of any of the roms, they were downloaded from internet archive)

## Controles / Controls
O teclado original do Chip-8, com 16 teclas, foi mapeado para o teclado QWERTY da seguinte forma:
The original Chip-8 keyboard, with 16 keys, mapped to QWERTY the following way:

Chip-8 - QWERTY  
1	- 1  
2	- 2  
3	- 3  
C	- 4  
4	- Q  
5	- W  
6	- E  
D	- R  
7	- A  
8	- S  
9	- D  
E	- F  
A	- Z  
0	- X  
B	- C  
F	- V  

