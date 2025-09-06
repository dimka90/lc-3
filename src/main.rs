use std::{io::ErrorKind, u16};

#[derive(Debug)]
#[repr(u16)]
enum Registers {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT,
}

#[derive(Debug)]
#[repr(u16)]
enum Opcodes {
    BR,
    ADD,  /* add  */
    LD,   /* load */
    ST,   /* store */
    JSR,  /* jump register */
    AND,  /* bitwise and */
    LDR,  /* load register */
    STR,  /* store register */
    RTI,  /* unused */
    NOT,  /* bitwise not */
    LDI,  /* load indirect */
    STI,  /* store indirect */
    JMP,  /* jump */
    RES,  /* reserved (unused) */
    LEA,  /* load effective address */
    TRAP, /* execute trap */
}

#[derive(Debug)]

enum Flags {
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}

const MEMORY_MAX: usize = 1 << 16;
const REG_LOCATIONS: u16 = 10;
const PC_START: u16 = 0x3000;

#[derive(Debug)]
struct Memory {
    locations: [u16; MEMORY_MAX as usize],
}

#[derive(Debug)]
struct Register {
    locations: [u16; REG_LOCATIONS as usize],
}

trait RegisterTrait {
    fn new() -> Self;
    fn load(&self, reg_location: Registers) -> Option<u16>;
    fn store(&mut self, instr: u16, reg_location: Registers) -> Result<(), ErrorKind>;
}

trait MemoryTrait {
    fn new() -> Self;
    fn mem_read(&self, register: &mut Register) -> Option<u16>;
}
impl Opcodes{
    fn to_u16(op:u16)->Option<Opcodes>{
        match op {
            0=>Some(Opcodes::BR),
            1=>Some(Opcodes::ADD),
            2=>Some(Opcodes::LD),
            3=>Some(Opcodes::ST),
            4=>Some(Opcodes::JSR),
            5=>Some(Opcodes::AND),
            6=>Some(Opcodes::LDR),
            7=>Some(Opcodes::STR),
            8=>Some(Opcodes::RTI),
            9=>Some(Opcodes::NOT),
            10=>Some(Opcodes::LDI),
            11=>Some(Opcodes::STI),
            12=>Some(Opcodes::JMP),
            13=>Some(Opcodes::RES),
            14=>Some(Opcodes::LEA),
            15=>Some(Opcodes::TRAP),
            _=>None

        }
    }
}
impl RegisterTrait for Register {
    fn new() -> Self {
        Self {
            locations: [0u16; REG_LOCATIONS as usize],
        }
    }

    fn load(&self, reg_location: Registers) -> Option<u16> {
        let data = self.locations[reg_location as usize];
        return Some(data);
    }
    fn store(&mut self, instr: u16, reg_location: Registers) -> Result<(), ErrorKind> {
        self.locations[reg_location as usize] = instr;
        return Ok(());
    }
}

impl MemoryTrait for Memory {
    fn new() -> Self {
        Self {
            locations: [0u16; MEMORY_MAX as usize],
        }
    }

    fn mem_read(&self, register: &mut Register) -> Option<u16> {
        let mem_addr = register.locations[Registers::PC as usize];
        let _ = register.store(mem_addr + 1, Registers::PC);
        Some(self.locations[mem_addr as usize])
    }
}
fn convert_opcode(opcode:Opcodes)->u16{
     opcode as u16
}
const OPCODE_ADD:u16=Opcodes::ADD as u16;
fn main() {
    let mut memory = Memory::new();
    let mut register = Register::new();

    memory.locations[12288] = 0x3000;
    let _ = register.store(PC_START, Registers::PC);
    let instr = memory.mem_read(&mut register).unwrap();

    let opcode = instr >> 12;


    println!("{register:?}");
     println!("{:#01x}", 12288);
     println!("{opcode}");
     println!("instruction {instr:#01x}");
    

    match Opcodes::to_u16(opcode) {
        Some(Opcodes::ADD)  => println!("Print Addition Operation"),
        Some(Opcodes::AND) => println!("AND Operation"),
        Some(Opcodes::NOT) => println!("NOT operation"),
        Some(Opcodes::BR)  => println!("BR operation"),
        Some(Opcodes::JMP)  => println!("JMP operation"),
        Some(Opcodes::JSR)  => println!("JSR operation"),
        Some(Opcodes::LD)  => println!("LD operation"),
        Some(Opcodes::LDI)  => println!("LDI operation"),
        Some(Opcodes::LDR)  => println!("LDR operation"),
        Some(Opcodes::LEA)  => println!("LEA operation"),
        Some(Opcodes::ST)  => println!("ST operation"),
        Some(Opcodes::STI)  => println!("STI operation"),
        Some(Opcodes::STR)  => println!("STR operation"),
        Some(Opcodes::TRAP)  => println!("TRAP operation"),
        Some(Opcodes::RES)  => println!("RES operation"),
        Some(Opcodes::RTI)  => println!("RTI operation"),

        _ => println!("Unknown.......")
    }
}