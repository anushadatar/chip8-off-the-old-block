// Program associated with creating and testing the CPU needed for a 
// chip8 emulator.

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
    // TODO Docs
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

    // TODO Docs 
    pub fn cycle(&mut self) {
        self.get_next_opcode();
        // TODO Implement -> this should use a switch and fn implementations for each value
        // self.execute_opcode();
        // TODO handle delays.
        // self.execute_delay();
    }

    // Sets the cpu's opcode attribute to the next 16-bit opcode in the program
    // (based on the current program counter attribute).
    fn get_next_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    // TODO Docs
    fn load_program(&mut self, input_data: Vec<u8>) {
        let mut data = vec![0; 0x200];
        // TODO might need to add some padding here.
        for byte in input_data {
            data.push(byte)
        }
        for (index, &byte) in data.iter().enumerate() {
            self.memory[index] = byte;
        }
    }
    
    fn execute_opcode(&mut self) {
        match self.opcode & 0xf000 {
            // TODO add all of the opcodes and functions associated with them.
            _      => self.opcode_not_implemented()
        }
    }

    // Print to the console if a specified opcode has no implementation.
    // Exit after printing the value of the problematic opcode.
    fn opcode_not_implemented(&self) {
        println!("Specified opcode {:X} has no implementation. Exiting.", self.opcode);
        process::exit(0);
    }

}    

// Instantiate using the main function.
fn main() {
    let cpu = &mut CPU::new();
}

// TODO Add tests! 
