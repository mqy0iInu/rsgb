use crate::cpu::*;
use std::pin::Pin;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub enum OpCode {
    // Load/Store Operations
    LD, LDI, LDD, PUSH, POP,
    // Arithmetic/Logical Operations
    ADD, AND, SUB, OR, XOR, ADC, SBC, CP, INC, DEC, DAA, CPL,
    // Shift and Rotate Operations
    RLCA, RLA, RRCA, RRA,
    // Jump and Call Operations
    JP, JR, CALL, RET, RETI, RST,
    // Other
    NOP, CCF, SCF, HALT, STOP, DI, EI,
    // Prefix(CB)
    PrefixCB,
    // Undefined OP
    INVALID,
}

#[derive(Clone)]
pub enum OpCodeCB {
    // Shift and Rotate Operations(CB)
    RLC, RRC, RL, RR, SLA, SRA, SRL, SWAP,
    // Bit Operations(CB)
    BIT, SET, RES,
    // Undefined OP
    INVALID,
}

#[derive(Clone)]
pub enum Addressing {
    IMM,        // Immediate value
    IMPL,
    REG,        // Register (A, B, C, D, E, H, L)
    IND,        // Indirect (HL)
    IndInc,    // Indirect with auto-increment (HL+)
    IndDec,    // Indirect with auto-decrement (HL-)
    INDX,       // Indexed with immediate value (e.g., LD A, (nn))
    ABS,        // Absolute (nn)
    AbsIdx,    // Absolute indexed (e.g., LD A, (HL))
    REL,        // Relative (PC + n)
    BIT,        // Bit addressing (e.g., BIT b, r)
    REG16,
    CondImm,
    COND,
    IoC,
    IoImm,
    Reg16Imm,
    SpImm,
    SpR8,
}

#[derive(Clone)]
pub struct Instruction
{
    pub op_code: OpCode,
    pub op_rand: [u8; 2],
    pub cb: bool,
    pub op_code_cb: OpCodeCB,
    pub cycle: u8,
    pub addr_mode: Addressing,
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            op_code: OpCode::NOP,
            op_rand: [0; 2],
            cb: false,
            op_code_cb: OpCodeCB::INVALID,
            cycle: 0,
            addr_mode: Addressing::IMM,
        }
    }

// static mut S_CPU_INSTRUCTION: Lazy<Pin<Box<Instruction>>> = Lazy::new(|| {
//     let cpu_inst = Box::pin(Instruction::new());
//     cpu_inst
// });

    pub fn fetch_instruction(&mut self) -> u8 {
        let op_code = cpu_mem_read(get_cpu_reg(REG_PC));
        op_code
    }

    pub fn decode_instruction(&mut self, op_code: u8) {
        unsafe {
            match op_code.into() {
                0x00 => { self.op_code = OpCode::NOP; self.addr_mode = Addressing::IMPL },
                0x01 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG;}, // LD BC, nn
                0x02 => { self.op_code = OpCode::LDI; self.addr_mode = Addressing::IND; }, // LD (BC), A
                0x03 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC BC
                0x04 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC B
                0x05 => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC B
                0x06 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IMM; }, // LD B, n
                0x07 => { self.op_code = OpCode::RLCA; self.addr_mode = Addressing::IMPL }, // RLCA
                0x08 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::ABS; }, // LD (nn), SP
                0x09 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD HL, BC
                0x0A => { self.op_code = OpCode::LDI; self.addr_mode = Addressing::IND; }, // LD A, (BC)
                0x0B => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC BC
                0x0C => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC C
                0x0D => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC C
                0x0E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IMM; }, // LD C, n
                0x0F => { self.op_code = OpCode::RRCA; self.addr_mode = Addressing::IMPL }, // RRCA
                0x10 => { self.op_code = OpCode::STOP; self.addr_mode = Addressing::IMM; }, // STOP 0
                0x11 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD DE, nn
                0x12 => { self.op_code = OpCode::LDI; self.addr_mode = Addressing::IND; }, // LD (DE), A
                0x13 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC DE
                0x14 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC D
                0x15 => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC D
                0x16 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IMM; }, // LD D, n
                0x17 => { self.op_code = OpCode::RLA; self.addr_mode = Addressing::IMPL }, // RLA
                0x18 => { self.op_code = OpCode::JR; self.addr_mode = Addressing::REL; }, // JR n
                0x19 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD HL, DE
                0x1A => { self.op_code = OpCode::LDI; self.addr_mode = Addressing::IND; }, // LD A, (DE)
                0x1B => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC DE
                0x1C => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC E
                0x1D => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC E
                0x1E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IMM; }, // LD E, n
                0x1F => { self.op_code = OpCode::RRA; self.addr_mode = Addressing::IMPL }, // RRA
                0x20 => { self.op_code = OpCode::JR; self.addr_mode = Addressing::REL; }, // JR NZ, n
                0x21 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD HL, nn
                0x22 => { self.op_code = OpCode::LDI; self.addr_mode = Addressing::IndInc; }, // LDI (HL+), A
                0x23 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC HL
                0x24 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC H
                0x25 => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC H
                0x26 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IMM; }, // LD H, n
                0x27 => { self.op_code = OpCode::DAA; self.addr_mode = Addressing::IMPL }, // DAA
                0x28 => { self.op_code = OpCode::JR; self.addr_mode = Addressing::REL; }, // JR Z, n
                0x29 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD HL, HL
                0x2A => { self.op_code = OpCode::LDI; self.addr_mode = Addressing::IndInc; }, // LDI A, (HL+)
                0x2B => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC HL
                0x2C => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC L
                0x2D => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC L
                0x2E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IMM; }, // LD L, n
                0x2F => { self.op_code = OpCode::CPL; self.addr_mode = Addressing::IMPL }, // CPL
                0x30 => { self.op_code = OpCode::JR; self.addr_mode = Addressing::REL; }, // JR NC, n
                0x31 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD SP, nn
                0x32 => { self.op_code = OpCode::LDD; self.addr_mode = Addressing::IndDec; }, // LDD (HL-), A
                0x33 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC SP
                0x34 => { self.op_code = OpCode::INC; self.addr_mode = Addressing::IND; }, // INC (HL)
                0x35 => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::IND; }, // DEC (HL)
                0x36 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::INDX; }, // LD (HL), n
                0x37 => { self.op_code = OpCode::SCF; self.addr_mode = Addressing::IMPL }, // SCF
                0x38 => { self.op_code = OpCode::JR; self.addr_mode = Addressing::REL; }, // JR C, n
                0x39 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD HL, SP
                0x3A => { self.op_code = OpCode::LDD; self.addr_mode = Addressing::IndDec; }, // LDD A, (HL-)
                0x3B => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC SP
                0x3C => { self.op_code = OpCode::INC; self.addr_mode = Addressing::REG; }, // INC A
                0x3D => { self.op_code = OpCode::DEC; self.addr_mode = Addressing::REG; }, // DEC A
                0x3E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IMM; }, // LD A, n
                0x3F => { self.op_code = OpCode::CCF; self.addr_mode = Addressing::IMPL }, // CCF
                0x40 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD B, B
                0x41 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD B, C
                0x42 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD B, D
                0x43 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD B, E
                0x44 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD B, H
                0x45 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD B, L
                0x46 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD B, (HL)
                0x47 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD B, A
                0x48 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD C, B
                0x49 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD C, C
                0x4A => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD C, D
                0x4B => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD C, E
                0x4C => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD C, H
                0x4D => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD C, L
                0x4E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD C, (HL)
                0x4F => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD C, A
                0x50 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD D, B
                0x51 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD D, C
                0x52 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD D, D
                0x53 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD D, E
                0x54 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD D, H
                0x55 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD D, L
                0x56 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD D, (HL)
                0x57 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD D, A
                0x58 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD E, B
                0x59 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD E, C
                0x5A => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD E, D
                0x5B => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD E, E
                0x5C => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD E, H
                0x5D => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD E, L
                0x5E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD E, (HL)
                0x5F => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD E, A
                0x60 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD H, B
                0x61 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD H, C
                0x62 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD H, D
                0x63 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD H, E
                0x64 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD H, H
                0x65 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD H, L
                0x66 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD H, (HL)
                0x67 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD H, A
                0x68 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD L, B
                0x69 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD L, C
                0x6A => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD L, D
                0x6B => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD L, E
                0x6C => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD L, H
                0x6D => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD L, L
                0x6E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD L, (HL)
                0x6F => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD L, A
                0x70 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD (HL), B
                0x71 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD (HL), C
                0x72 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD (HL), D
                0x73 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD (HL), E
                0x74 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD (HL), H
                0x75 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD (HL), L
                0x76 => { self.op_code = OpCode::HALT; self.addr_mode = Addressing::IMPL }, // HALT
                0x77 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IND; }, // LD (HL), A
                0x78 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD A, B
                0x79 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD A, C
                0x7A => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD A, D
                0x7B => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD A, E
                0x7C => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD A, H
                0x7D => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD A, L
                0x7E => { self.op_code = OpCode::LD; self.addr_mode = Addressing::AbsIdx; }, // LD A, (HL)
                0x7F => { self.op_code = OpCode::LD; self.addr_mode = Addressing::REG; }, // LD A, A
                0x80 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD A, B
                0x81 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD A, C
                0x82 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD A, D
                0x83 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD A, E
                0x84 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD A, H
                0x85 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD A, L
                0x86 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::IND; }, // ADD A, (HL)
                0x87 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::REG; }, // ADD A, A
                0x88 => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::REG; }, // ADC A, B
                0x89 => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::REG; }, // ADC A, C
                0x8A => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::REG; }, // ADC A, D
                0x8B => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::REG; }, // ADC A, E
                0x8C => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::REG; }, // ADC A, H
                0x8D => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::REG; }, // ADC A, L
                0x8E => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::IND; }, // ADC A, (HL)
                0x8F => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::REG; }, // ADC A, A
                0x90 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::REG; }, // SUB B
                0x91 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::REG; }, // SUB C
                0x92 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::REG; }, // SUB D
                0x93 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::REG; }, // SUB E
                0x94 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::REG; }, // SUB H
                0x95 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::REG; }, // SUB L
                0x96 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::IND; }, // SUB (HL)
                0x97 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::REG; }, // SUB A
                0x98 => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::REG; }, // SBC A, B
                0x99 => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::REG; }, // SBC A, C
                0x9A => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::REG; }, // SBC A, D
                0x9B => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::REG; }, // SBC A, E
                0x9C => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::REG; }, // SBC A, H
                0x9D => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::REG; }, // SBC A, L
                0x9E => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::IND; }, // SBC A, (HL)
                0x9F => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::REG; }, // SBC A, A
                0xA0 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::REG; }, // AND B
                0xA1 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::REG; }, // AND C
                0xA2 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::REG; }, // AND D
                0xA3 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::REG; }, // AND E
                0xA4 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::REG; }, // AND H
                0xA5 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::REG; }, // AND L
                0xA6 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::IND; }, // AND (HL)
                0xA7 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::REG; }, // AND A
                0xA8 => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::REG; }, // XOR B
                0xA9 => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::REG; }, // XOR C
                0xAA => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::REG; }, // XOR D
                0xAB => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::REG; }, // XOR E
                0xAC => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::REG; }, // XOR H
                0xAD => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::REG; }, // XOR L
                0xAE => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::IND; }, // XOR (HL)
                0xAF => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::REG; }, // XOR A
                0xB0 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::REG; }, // OR B
                0xB1 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::REG; }, // OR C
                0xB2 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::REG; }, // OR D
                0xB3 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::REG; }, // OR E
                0xB4 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::REG; }, // OR H
                0xB5 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::REG; }, // OR L
                0xB6 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::IND; }, // OR (HL)
                0xB7 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::REG; }, // OR A
                0xB8 => { self.op_code = OpCode::CP; self.addr_mode = Addressing::REG; }, // CP B
                0xB9 => { self.op_code = OpCode::CP; self.addr_mode = Addressing::REG; }, // CP C
                0xBA => { self.op_code = OpCode::CP; self.addr_mode = Addressing::REG; }, // CP D
                0xBB => { self.op_code = OpCode::CP; self.addr_mode = Addressing::REG; }, // CP E
                0xBC => { self.op_code = OpCode::CP; self.addr_mode = Addressing::REG; }, // CP H
                0xBD => { self.op_code = OpCode::CP; self.addr_mode = Addressing::REG; }, // CP L
                0xBE => { self.op_code = OpCode::CP; self.addr_mode = Addressing::IND; }, // CP (HL)
                0xBF => { self.op_code = OpCode::CP; self.addr_mode = Addressing::REG; }, // CP A
                0xC0 => { self.op_code = OpCode::RET; self.addr_mode = Addressing::COND; }, // RET NZ
                0xC1 => { self.op_code = OpCode::POP; self.addr_mode = Addressing::REG16; }, // POP BC
                0xC2 => { self.op_code = OpCode::JP; self.addr_mode = Addressing::CondImm; }, // JP NZ, a16
                0xC3 => { self.op_code = OpCode::JP; self.addr_mode = Addressing::IMM }, // JP a16
                0xC4 => { self.op_code = OpCode::CALL; self.addr_mode = Addressing::CondImm; }, // CALL NZ, a16
                0xC5 => { self.op_code = OpCode::PUSH; self.addr_mode = Addressing::REG16; }, // PUSH BC
                0xC6 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::IMM }, // ADD A, d8
                0xC7 => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 00H
                0xC8 => { self.op_code = OpCode::RET; self.addr_mode = Addressing::COND; }, // RET Z
                0xC9 => { self.op_code = OpCode::RET; self.addr_mode = Addressing::IMPL }, // RET
                0xCA => { self.op_code = OpCode::JP; self.addr_mode = Addressing::CondImm; }, // JP Z, a16
                0xCB => { // Prefix CB
                    self.op_code = OpCode::PrefixCB;
                    let op_code_cb = cpu_mem_read(get_cpu_reg(REG_PC).wrapping_add(1));
                    self.decode_cb_instruction(op_code_cb);
                },
                0xCC => { self.op_code = OpCode::CALL; self.addr_mode = Addressing::CondImm; }, // CALL Z, a16
                0xCD => { self.op_code = OpCode::CALL; self.addr_mode = Addressing::IMM }, // CALL a16
                0xCE => { self.op_code = OpCode::ADC; self.addr_mode = Addressing::IMM }, // ADC A, d8
                0xCF => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 08H
                0xD0 => { self.op_code = OpCode::RET; self.addr_mode = Addressing::COND;}, // RET NC
                0xD1 => { self.op_code = OpCode::POP; self.addr_mode = Addressing::REG16;}, // POP DE
                0xD2 => { self.op_code = OpCode::JP; self.addr_mode = Addressing::CondImm;}, // JP NC, a16
                0xD4 => { self.op_code = OpCode::CALL; self.addr_mode = Addressing::CondImm;}, // CALL NC, a16
                0xD5 => { self.op_code = OpCode::PUSH; self.addr_mode = Addressing::REG16;}, // PUSH DE
                0xD6 => { self.op_code = OpCode::SUB; self.addr_mode = Addressing::IMM }, // SUB d8
                0xD7 => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 10H
                0xD8 => { self.op_code = OpCode::RET; self.addr_mode = Addressing::COND;}, // RET C
                0xD9 => { self.op_code = OpCode::RETI; self.addr_mode = Addressing::IMPL }, // RETI
                0xDA => { self.op_code = OpCode::JP; self.addr_mode = Addressing::CondImm;}, // JP C, a16
                0xDC => { self.op_code = OpCode::CALL; self.addr_mode = Addressing::CondImm;}, // CALL C, a16
                0xDE => { self.op_code = OpCode::SBC; self.addr_mode = Addressing::IMM }, // SBC A, d8
                0xDF => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 18H
                0xE0 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IoImm;}, // LDH (a8), A
                0xE1 => { self.op_code = OpCode::POP; self.addr_mode = Addressing::REG16;}, // POP HL
                0xE2 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IoC;}, // LD (C), A
                0xE5 => { self.op_code = OpCode::PUSH; self.addr_mode = Addressing::REG16;}, // PUSH HL
                0xE6 => { self.op_code = OpCode::AND; self.addr_mode = Addressing::IMM }, // AND d8
                0xE7 => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 20H
                0xE8 => { self.op_code = OpCode::ADD; self.addr_mode = Addressing::SpImm }, // ADD SP, r8
                0xE9 => { self.op_code = OpCode::JP; self.addr_mode = Addressing::REG16;}, // JP (HL)
                0xEA => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IoImm; }, // LD (a16), A
                0xEE => { self.op_code = OpCode::XOR; self.addr_mode = Addressing::IMM }, // XOR d8
                0xEF => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 28H
                0xF0 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IoImm; }, // LDH A, (a8)
                0xF1 => { self.op_code = OpCode::POP; self.addr_mode = Addressing::REG16;}, // POP AF
                0xF2 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IoC;  }, // LD A, (C)
                0xF3 => { self.op_code = OpCode::DI; self.addr_mode = Addressing::IMPL }, // DI
                0xF5 => { self.op_code = OpCode::PUSH; self.addr_mode = Addressing::REG16;}, // PUSH AF
                0xF6 => { self.op_code = OpCode::OR; self.addr_mode = Addressing::IMM }, // OR d8
                0xF7 => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 30H
                0xF8 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::SpR8 }, // LD HL, SP+r8
                0xF9 => { self.op_code = OpCode::LD; self.addr_mode = Addressing::Reg16Imm;}, // LD SP, HL
                0xFA => { self.op_code = OpCode::LD; self.addr_mode = Addressing::IoImm;  }, // LD A, (a16)
                0xFB => { self.op_code = OpCode::EI; self.addr_mode = Addressing::IMPL }, // EI
                0xFE => { self.op_code = OpCode::CP; self.addr_mode = Addressing::IMM }, // CP d8
                0xFF => { self.op_code = OpCode::RST; self.addr_mode = Addressing::IMM; }, // RST 38H
                _ => { self.op_code = OpCode::INVALID },
            }
        }
    }

    pub fn decode_cb_instruction(&mut self, op_code_cb: u8) {
        unsafe {
            match op_code_cb {
                0x00 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::REG }, // RLC B
                0x01 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::REG }, // RLC C
                0x02 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::REG }, // RLC D
                0x03 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::REG }, // RLC E
                0x04 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::REG }, // RLC H
                0x05 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::REG }, // RLC L
                0x06 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::IND }, // RLC (HL)
                0x07 => { self.op_code_cb = OpCodeCB::RLC; self.addr_mode = Addressing::REG }, // RLC A
                0x08 => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::REG }, // RRC B
                0x09 => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::REG }, // RRC C
                0x0A => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::REG }, // RRC D
                0x0B => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::REG }, // RRC E
                0x0C => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::REG }, // RRC H
                0x0D => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::REG }, // RRC L
                0x0E => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::IND }, // RRC (HL)
                0x0F => { self.op_code_cb = OpCodeCB::RRC; self.addr_mode = Addressing::REG }, // RRC A
                0x10 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::REG },  // RL B
                0x11 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::REG },  // RL C
                0x12 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::REG },  // RL D
                0x13 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::REG },  // RL E
                0x14 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::REG },  // RL H
                0x15 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::REG },  // RL L
                0x16 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::IND },  // RL (HL)
                0x17 => { self.op_code_cb = OpCodeCB::RL; self.addr_mode = Addressing::REG },  // RL A
                0x18 => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::REG },  // RR B
                0x19 => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::REG },  // RR C
                0x1A => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::REG },  // RR D
                0x1B => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::REG },  // RR E
                0x1C => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::REG },  // RR H
                0x1D => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::REG },  // RR L
                0x1E => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::IND },  // RR (HL)
                0x1F => { self.op_code_cb = OpCodeCB::RR; self.addr_mode = Addressing::REG },  // RR A
                0x20 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::REG }, // SLA B
                0x21 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::REG }, // SLA C
                0x22 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::REG }, // SLA D
                0x23 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::REG }, // SLA E
                0x24 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::REG }, // SLA H
                0x25 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::REG }, // SLA L
                0x26 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::IND }, // SLA (HL)
                0x27 => { self.op_code_cb = OpCodeCB::SLA; self.addr_mode = Addressing::REG }, // SLA A
                0x28 => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::REG }, // SRA B
                0x29 => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::REG }, // SRA C
                0x2A => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::REG }, // SRA D
                0x2B => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::REG }, // SRA E
                0x2C => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::REG }, // SRA H
                0x2D => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::REG }, // SRA L
                0x2E => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::IND }, // SRA (HL)
                0x2F => { self.op_code_cb = OpCodeCB::SRA; self.addr_mode = Addressing::REG }, // SRA A
                0x30 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::REG }, // SWAP B
                0x31 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::REG }, // SWAP C
                0x32 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::REG }, // SWAP D
                0x33 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::REG }, // SWAP E
                0x34 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::REG }, // SWAP H
                0x35 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::REG }, // SWAP L
                0x36 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::IND }, // SWAP (HL)
                0x37 => { self.op_code_cb = OpCodeCB::SWAP; self.addr_mode = Addressing::REG }, // SWAP A
                0x38 => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::REG }, // SRL B
                0x39 => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::REG }, // SRL C
                0x3A => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::REG }, // SRL D
                0x3B => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::REG }, // SRL E
                0x3C => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::REG }, // SRL H
                0x3D => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::REG }, // SRL L
                0x3E => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::IND }, // SRL (HL)
                0x3F => { self.op_code_cb = OpCodeCB::SRL; self.addr_mode = Addressing::REG }, // SRL A
                0x40 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 0, B
                0x41 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 0, C
                0x42 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 0, D
                0x43 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 0, E
                0x44 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 0, H
                0x45 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 0, L
                0x46 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 0, (HL)
                0x47 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 0, A
                0x48 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 1, B
                0x49 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 1, C
                0x4A => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 1, D
                0x4B => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 1, E
                0x4C => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 1, H
                0x4D => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 1, L
                0x4E => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 1, (HL)
                0x4F => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 1, A
                0x50 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 2, B
                0x51 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 2, C
                0x52 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 2, D
                0x53 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 2, E
                0x54 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 2, H
                0x55 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 2, L
                0x56 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 2, (HL)
                0x57 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 2, A
                0x58 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 3, B
                0x59 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 3, C
                0x5A => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 3, D
                0x5B => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 3, E
                0x5C => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 3, H
                0x5D => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 3, L
                0x5E => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 3, (HL)
                0x5F => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 3, A
                0x60 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 4, B
                0x61 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 4, C
                0x62 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 4, D
                0x63 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 4, E
                0x64 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 4, H
                0x65 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 4, L
                0x66 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 4, (HL)
                0x67 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 4, A
                0x68 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 5, B
                0x69 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 5, C
                0x6A => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 5, D
                0x6B => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 5, E
                0x6C => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 5, H
                0x6D => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 5, L
                0x6E => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 5, (HL)
                0x6F => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 5, A
                0x70 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 6, B
                0x71 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 6, C
                0x72 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 6, D
                0x73 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 6, E
                0x74 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 6, H
                0x75 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 6, L
                0x76 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 6, (HL)
                0x77 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 6, A
                0x78 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 7, B
                0x79 => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 7, C
                0x7A => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 7, D
                0x7B => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 7, E
                0x7C => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 7, H
                0x7D => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 7, L
                0x7E => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::IND }, // BIT 7, (HL)
                0x7F => { self.op_code_cb = OpCodeCB::BIT; self.addr_mode = Addressing::BIT }, // BIT 7, A
                0x80 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 0, B
                0x81 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 0, C
                0x82 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 0, D
                0x83 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 0, E
                0x84 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 0, H
                0x85 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 0, L
                0x86 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 0, (HL)
                0x87 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 0, A
                0x88 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 1, B
                0x89 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 1, C
                0x8A => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 1, D
                0x8B => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 1, E
                0x8C => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 1, H
                0x8D => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 1, L
                0x8E => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 1, (HL)
                0x8F => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 1, A
                0x90 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 2, B
                0x91 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 2, C
                0x92 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 2, D
                0x93 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 2, E
                0x94 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 2, H
                0x95 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 2, L
                0x96 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 2, (HL)
                0x97 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 2, A
                0x98 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 3, B
                0x99 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 3, C
                0x9A => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 3, D
                0x9B => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 3, E
                0x9C => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 3, H
                0x9D => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 3, L
                0x9E => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 3, (HL)
                0x9F => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 3, A
                0xA0 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 4, B
                0xA1 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 4, C
                0xA2 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 4, D
                0xA3 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 4, E
                0xA4 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 4, H
                0xA5 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 4, L
                0xA6 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 4, (HL)
                0xA7 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 4, A
                0xA8 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 5, B
                0xA9 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 5, C
                0xAA => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 5, D
                0xAB => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 5, E
                0xAC => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 5, H
                0xAD => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 5, L
                0xAE => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 5, (HL)
                0xAF => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 5, A
                0xB0 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 6, B
                0xB1 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 6, C
                0xB2 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 6, D
                0xB3 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 6, E
                0xB4 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 6, H
                0xB5 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 6, L
                0xB6 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 6, (HL)
                0xB7 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 6, A
                0xB8 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 7, B
                0xB9 => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 7, C
                0xBA => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 7, D
                0xBB => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 7, E
                0xBC => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 7, H
                0xBD => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 7, L
                0xBE => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::IND }, // RES 7, (HL)
                0xBF => { self.op_code_cb = OpCodeCB::RES; self.addr_mode = Addressing::BIT }, // RES 7, A
                0xC0 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 0, B
                0xC1 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 0, C
                0xC2 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 0, D
                0xC3 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 0, E
                0xC4 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 0, H
                0xC5 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 0, L
                0xC6 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 0, (HL)
                0xC7 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 0, A
                0xC8 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 1, B
                0xC9 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 1, C
                0xCA => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 1, D
                0xCB => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 1, E
                0xCC => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 1, H
                0xCD => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 1, L
                0xCE => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 1, (HL)
                0xCF => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 1, A
                0xD0 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 2, B
                0xD1 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 2, C
                0xD2 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 2, D
                0xD3 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 2, E
                0xD4 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 2, H
                0xD5 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 2, L
                0xD6 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 2, (HL)
                0xD7 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 2, A
                0xD8 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 3, B
                0xD9 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 3, C
                0xDA => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 3, D
                0xDB => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 3, E
                0xDC => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 3, H
                0xDD => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 3, L
                0xDE => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 3, (HL)
                0xDF => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 3, A
                0xE0 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 4, B
                0xE1 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 4, C
                0xE2 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 4, D
                0xE3 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 4, E
                0xE4 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 4, H
                0xE5 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 4, L
                0xE6 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 4, (HL)
                0xE7 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 4, A
                0xE8 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 5, B
                0xE9 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 5, C
                0xEA => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 5, D
                0xEB => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 5, E
                0xEC => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 5, H
                0xED => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 5, L
                0xEE => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 5, (HL)
                0xEF => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 5, A
                0xF0 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 6, B
                0xF1 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 6, C
                0xF2 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 6, D
                0xF3 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 6, E
                0xF4 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 6, H
                0xF5 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 6, L
                0xF6 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 6, (HL)
                0xF7 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 6, A
                0xF8 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 7, B
                0xF9 => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 7, C
                0xFA => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 7, D
                0xFB => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 7, E
                0xFC => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 7, H
                0xFD => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 7, L
                0xFE => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::IND }, // SET 7, (HL)
                0xFF => { self.op_code_cb = OpCodeCB::SET; self.addr_mode = Addressing::BIT }, // SET 7, A
            }
        }
    }

    pub fn execute_instruction(&mut self)
    {
        // TODO :CPU OP Exec
    }

    pub fn execute_cb_instruction(&mut self)
    {
        // TODO :CB拡張命令 Exec
    }
}