#[derive(Debug)]
#[repr(u16)]
 enum Registers{
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
    COUNT
    }

#[derive(Debug)]
enum Opcodes
{
    BR,
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP    /* execute trap */
}

#[derive(Debug)]

enum Flags
{
    POS = 1 << 0, /* P */
    ZRO = 1 << 1, /* Z */
    NEG = 1 << 2, /* N */
}


const MEMORY_MAX: u16 = u16::MAX;
// const MEMORY_MAX : u16 = ((1 << 16) - 1) as u16;
// const MEMORY_MAX : usize = 1 << 16;


#[derive(Debug)]
struct Memory {
    locations: [u16; MEMORY_MAX as usize]
}

impl Memory {
    fn new() -> Self {
        Self {
            locations: [0u16; MEMORY_MAX as usize]
        }
    }

    
}
// const MEMORY_MAX : u16 = ((1u32 << 16) - 1 as u16);


fn main() {
    // let MEMORY_MAX : u16 = ((1u32 << 16) - 1).try_into().unwrap();

    const PC_START: u16 = 0x3000;
    
    let  mut reg_storage: [u16; Registers::COUNT as usize] =  [0; Registers::COUNT as usize];
    
    let ZERO_FLAG = Flags::ZRO as u16;
    let register_cond = Registers::COND as usize;

    reg_storage[register_cond] = ZERO_FLAG;
   

    reg_storage[Registers::PC as usize] = PC_START;
    // println!("Registers {:?}", reg_storage);
    let mut memory = Memory::new();

    // memory.Location.push(8);
    println!("{:?}", memory.locations[MEMORY_MAX as usize]);



}