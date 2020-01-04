// Create a cpu struct that matches chip8 specs.
struct CPU {
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
impl CPU {
    fn new() -> CPU {
        CPU {
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

    // Load a program. Data starts at 0x200.
    fn load_prog(&mut self, prog:Vec<u8>) {
        let prog_data = &mut vec![0; 0x200];
        for current_byte in prog {
            prog_data.push(i);
        }
        for (current_addr, &current_byte) in prog_data.iter().enumerate() {
            self.memory[current_addr] = current_byte;
        }
    }      
}    

// Instantiate using the main function.
fn main() {
    let cpu = &mut CPU::new();
    cpu.load(vec![0x13, 0xC5]);
}

