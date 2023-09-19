use super::Loc;

#[derive(Debug, Clone)]
pub struct Token {
    pub loc: Loc,
    pub typ: TokenType,
    pub value: ValueType
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int(u32),
    String(String),
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    Operation(OpType),
    DataLabel(Vec<i16>),
    TextLabel(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OpType {
    NOP    = 0x01,
    MOVIA  = 0x02,
    MOVIB  = 0x03,
    MOVIC  = 0x04,
    MOVID  = 0x05,
    MOVAM  = 0x06,
    MOVBM  = 0x07,
    MOVCM  = 0x08,
    MOVDM  = 0x09,
    MOVIM  = 0x0A,
    PRNT   = 0x0B,
    ADD    = 0x0C,
    SUB    = 0x0D,
    INC    = 0x0E,
    DEC    = 0x0F,
    LDI    = 0x10,
    HLT    = 0x11,
    CALL   = 0x12,
    RET    = 0x13,
    AND    = 0x14,
    OR     = 0x15,
    XOR    = 0x16,
    JMP    = 0x17,
    JNE    = 0x18,
    JE     = 0x19,
    JZ     = 0x1A,
    JNZ    = 0x1B,
    JGT    = 0x1C,
    JGE    = 0x1D,
    JLT    = 0x1E,
    JLE    = 0x1F,
    JO     = 0x20,
    CMP    = 0x21,
    POPA   = 0x22,
    POPB   = 0x23,
    POPC   = 0x24,
    POPD   = 0x25,
    POPM   = 0x26,
    PUSHA  = 0x27,
    PUSHB  = 0x28,
    PUSHC  = 0x29,
    PUSHD  = 0x2A,
    PUSHM  = 0x2B,
    PUSHI  = 0x2C,
}

impl OpType {
    
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        let t = match s.to_lowercase().as_str() {
                "nop"    => OpType::NOP,
                "movia"  => OpType::MOVIA,
                "movib"  => OpType::MOVIB,
                "movic"  => OpType::MOVIC,
                "movid"  => OpType::MOVID,
                "movam"  => OpType::MOVAM,
                "movbm"  => OpType::MOVBM,
                "movcm"  => OpType::MOVCM,
                "movdm"  => OpType::MOVDM,
                "movim"  => OpType::MOVIM,
                "prnt"   => OpType::PRNT,
                "add"    => OpType::ADD,
                "sub"    => OpType::SUB,
                "inc"    => OpType::INC,
                "dec"    => OpType::DEC,
                "ldi"    => OpType::LDI,
                "hlt"    => OpType::HLT,
                "call"   => OpType::CALL,
                "ret"    => OpType::RET,
                "and"    => OpType::AND,
                "or"     => OpType::OR,
                "xor"    => OpType::XOR,
                "jmp"    => OpType::JMP,
                "jne"    => OpType::JNE,
                "je"     => OpType::JE,
                "jz"     => OpType::JZ,
                "jnz"    => OpType::JNZ,
                "jgt"    => OpType::JGT,
                "jge"    => OpType::JGE,
                "jlt"    => OpType::JLT,
                "jle"    => OpType::JLE,
                "jo"     => OpType::JO,
                "cmp"    => OpType::CMP,
                "popa"   => OpType::POPA,
                "popb"   => OpType::POPB,
                "popc"   => OpType::POPC,
                "popd"   => OpType::POPD,
                "popm"   => OpType::POPM,
                "pusha"  => OpType::PUSHA,
                "pushb"  => OpType::PUSHB,
                "pushc"  => OpType::PUSHC,
                "pushd"  => OpType::PUSHD,
                "pushm"  => OpType::PUSHM,
                "pushi"  => OpType::PUSHI,
            _ => return None
        };

        Some(t)
    }

    pub fn arg_type(self) -> ArgType {
        match self {
            OpType::NOP   => ArgType::None,
            OpType::MOVIA => ArgType::Immediate,
            OpType::MOVIB => ArgType::Immediate,
            OpType::MOVIC => ArgType::Immediate,
            OpType::MOVID => ArgType::Immediate,
            OpType::MOVAM => ArgType::Addr,
            OpType::MOVBM => ArgType::Addr,
            OpType::MOVCM => ArgType::Addr,
            OpType::MOVDM => ArgType::Addr,
            OpType::MOVIM => ArgType::Addr,
            OpType::PRNT  => ArgType::None,
            OpType::ADD   => ArgType::Any,
            OpType::SUB   => ArgType::Any,
            OpType::INC   => ArgType::Any,
            OpType::DEC   => ArgType::Any,
            OpType::LDI   => ArgType::Immediate,
            OpType::HLT   => ArgType::None,
            OpType::CALL  => ArgType::Label,
            OpType::RET   => ArgType::None,
            OpType::AND   => ArgType::Any,
            OpType::OR    => ArgType::Any,
            OpType::XOR   => ArgType::Any,
            OpType::JMP   => ArgType::Label,
            OpType::JNE   => ArgType::Label,
            OpType::JE    => ArgType::Label,
            OpType::JZ    => ArgType::Label,
            OpType::JNZ   => ArgType::Label,
            OpType::JGT   => ArgType::Label,
            OpType::JGE   => ArgType::Label,
            OpType::JLT   => ArgType::Label,
            OpType::JLE   => ArgType::Label,
            OpType::JO    => ArgType::Label,
            OpType::CMP   => ArgType::Any,
            OpType::POPA  => ArgType::None,
            OpType::POPB  => ArgType::None,
            OpType::POPC  => ArgType::None,
            OpType::POPD  => ArgType::None,
            OpType::POPM  => ArgType::Addr,
            OpType::PUSHA => ArgType::None,
            OpType::PUSHB => ArgType::None,
            OpType::PUSHC => ArgType::None,
            OpType::PUSHD => ArgType::None,
            OpType::PUSHM => ArgType::Addr,
            OpType::PUSHI => ArgType::Immediate,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum ArgType {
    Addr,
    Immediate,
    Any,
    Label,
    None,
}
