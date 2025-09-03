# CHIP8
## ENGLISH VERSION BELOW

# Emulador de Chip-8 em Rust
Um emulador de Chip-8, Super Chip-8 e XO-CHIP escrito em Rust, utilizando minifb para a criação da janela e renderização, e rodio para a reprodução de áudio. Este projeto foi criado com o objetivo de aprender mais sobre emulação e a linguagem Rust.

O que é o Chip-8?
Chip-8 é uma linguagem de programação interpretada, desenvolvida por Joseph Weisbecker. Foi inicialmente usada no microcomputador COSMAC VIP de 8 bits em meados da década de 1970. O Chip-8 é o que hoje chamaríamos de "fantasy console" ou "máquina virtual". Programas Chip-8 rodam em uma máquina virtual Chip-8. Nas décadas de 1970 e 1980, jogos para esta máquina virtual foram distribuídos em revistas de informática e como "type-in programs", onde o usuário digitava o código-fonte.

Atualmente, o Chip-8 é uma ótima maneira de começar a aprender sobre o desenvolvimento de emuladores, devido à sua simplicidade.

Funcionalidades
Emulação Completa de Chip-8: Suporte para todas as 35 opcodes originais.

Suporte a Super Chip-8 (SCHIP):

Modo de alta resolução (128x64 pixels).

Novas opcodes para rolagem de tela (para a direita, esquerda e para baixo).

Sprites maiores (16x16).

Suporte a XO-CHIP:

Novas instruções para salvar e carregar registradores.

Instrução de áudio aprimorada.

Áudio: Emissão de bipes sonoros através da biblioteca rodio.

Controles Mapeados: Mapeamento de teclado para uma experiência de jogo mais confortável em teclados modernos.

Como Executar
Para compilar e executar o emulador, você precisará ter o Rust e o Cargo instalados.

Clone o repositório:

Bash

git clone https://github.com/seu-usuario/seu-repositorio.git
cd seu-repositorio
Altere o caminho da ROM:
No arquivo src/main.rs, localize a seguinte linha e altere o caminho para a ROM que você deseja carregar:

Rust

let rom_path = "C:/Users/leohe/OneDrive/Documentos/chip8/chip8_emulator/Chip8-Database/SuperChip8-Games/Snow Daze (unknown author)(2015).sc8";
Compile e execute o projeto:

Bash

cargo run --release
O emulador iniciará e carregará a ROM especificada.

Controles
O teclado original do Chip-8, com 16 teclas, foi mapeado para o teclado QWERTY da seguinte forma:

Teclado Chip-8	Teclado QWERTY
1	1
2	2
3	3
C	4
4	Q
5	W
6	E
D	R
7	A
8	S
9	D
E	F
A	Z
0	X
B	C
F	V

Exportar para as Planilhas
Dependências
Este projeto utiliza as seguintes crates do Rust:

minifb: Para a criação de janelas e renderização de framebuffer.

rand: Para a geração de números aleatórios (opcode Cxkk).

rodio: Para a funcionalidade de áudio.

As dependências estão listadas no arquivo Cargo.toml.

Estrutura do Projeto
src/main.rs: Contém toda a lógica do emulador, incluindo a struct Chip8, o ciclo de emulação, a renderização e o tratamento de entrada.

Cargo.toml: Define as metadados e dependências do projeto.

Chip8-Database/: Contém uma coleção de ROMs de jogos para Chip-8, Super Chip-8 e XO-CHIP.

Opcodes Implementadas
O emulador implementa o conjunto completo de instruções do Chip-8, bem como as extensões do Super Chip-8 e XO-CHIP.

(Você pode adicionar aqui uma tabela com as opcodes implementadas, se desejar)

Contribuições
Contribuições são bem-vindas! Se você encontrar algum bug ou tiver alguma sugestão de melhoria, sinta-se à vontade para abrir uma issue ou um pull request.
