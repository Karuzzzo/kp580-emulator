
type Word = u16;
type Byte = u8;

struct Mem {
    MAX_MEM = 
}

struct CPU {
    //Registers
    PC: Word,
    SP: Word,
    A: Byte,
    //POH registers
    B: Byte,
    C: Byte,

    D: Byte,
    E: Byte,
    
    H: Byte,
    L: Byte,
    //only 5 bits (Z, CY, S, P, C)
    RP: Byte, 
}

impl CPU {
    fn new() -> Self {
        CPU{
            PC : 0xffff,
            SP : 0xffff,
            A : 0,
            B : 0,
            C : 0,
            D : 0,
            E : 0,
            H : 0,
            L : 0,
            RP : 0,            
        }
    }
    fn reset(&mut self){
        //check init params in emulator
        self.PC = 0xffff;
        self.SP = 0xffff;
        self.A = 0;
        self.B = 0;
        self.C = 0;
        self.D = 0;
        self.E = 0;
        self.H = 0;
        self.L = 0;
        self.RP = 0;
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.reset();

    println!("Hello, world!");
}
