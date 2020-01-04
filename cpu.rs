// Create a cpu struct that matches chip8 specs.
struct cpu {
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

// Create an impl for instantiation.
impl cpu {
    fn new() -> cpu {
        cpu {
            regi:[0;16],
            addr: 0x200,
            memory: [0; 4096],
            pc: 0x200,
            sp: 0,
            opcode: 0,
            sound_timer: 0,
            delay_timer: 0
        }
    }
}

