// Creating a cpu struct that matches chip8 specs.

struct cpu{
    // 16 8-bit register array, single reserved bit.
    regi: [u8; 16],
    // Memory address.
    addr: u16,
    // General memory.
    memory: [u8; 4096],
    // Program Counter
    pc: u16,
    // Stack Pointer.
    sp : u8,
    // Storage for opcode. 
    opcode: u16,
    // Timers.
    sound_timer: u8,
    delay_timer: u8
}
