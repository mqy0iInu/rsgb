use std::pin::Pin;
use once_cell::sync::Lazy;

const ZERO_FLG: u8 = 0b1000_0000;      // bit7: Z Flag.
const NEGATIVE_FLG: u8 = 0b0100_0000;  // bit6: N Flag.
const HALF_CARRY_FLG: u8 = 0b0010_0000; // bit5: H Flag.
const CARRY_FLG: u8 = 0b0001_0000;     // bit4: C Flag.

const BOOT_ROM_ADDR: u16 = 0x0000;

#[derive(Clone)]
pub enum OpCode {
    // Load/Store Operations
    LD, LDI, LDD, PUSH, POP,
    // Arithmetic/Logical Operations
    ADD, AND, SUB, OR, XOR, ADC, SBC, CP, INC, DEC, DAA, CPL,
    // Shift and Rotate Operations
    RLCA, RLA, RRCA, RRA,
    // Shift and Rotate Operations(CB)
    RLC, RRC, RL, RR, SLA, SRA, SRL, SWAP,
    // Jump and Call Operations
    JP, JR, CALL, RET, RETI, RST,
    // Bit Operations(CB)
    BIT, SET, RES,
    // Other
    NOP, CCF, SCF, HALT, STOP, DI, EI,
    PrefixCb,
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
pub struct LR35902
{
    pub reg_af: u16, // AF
    pub reg_bc: u16, // BC
    pub reg_de: u16, // DE
    pub reg_hl: u16, // HL

    pub reg_sp: u16,
    pub reg_pc: u16,

    pub op_code: OpCode,
    pub op_rand: [u8; 2],
    pub cb: bool,
    pub cycle: u8,
    pub addr_mode: Addressing,

    pub rst: bool,
    pub nmi: bool,
    pub irq: bool,

    pub cpu_run: bool,
}

impl LR35902 {
    pub fn new() -> Self {
        LR35902 {
            reg_af: 0x0000, // AF
            reg_bc: 0x0000, // BC
            reg_de: 0x0000, // DE
            reg_hl: 0x0000, // HL

            reg_sp: 0xFFFE,
            reg_pc: BOOT_ROM_ADDR,

            op_code: OpCode::NOP,
            op_rand: [0; 2],
            cb: false,
            cycle: 0,
            addr_mode: Addressing::IMM,

            rst: false,
            nmi: false,
            irq: false,

            cpu_run: false,
        }
    }

    fn cls_status_flg(&mut self, flg: u8) {
        let flag_mask = !(1u16 << (flg & 0x0F));
        self.reg_af &= (0xFF00 | flag_mask);
    }

    fn set_status_flg(&mut self, flg: u8) {
        let flag_value = 1u16 << (flg & 0x0F);
        self.reg_af |= flag_value;
    }

    fn get_status_flg(&self, flg: u8) -> bool {
        let flag_value = 1u16 << (flg & 0x0F);
        (self.reg_af & flag_value) != 0
    }

    fn read(&mut self, address: u16) -> u8
    {
        // TODO :Mem Read
        0
    }

    fn write(&mut self, address: u16, data: u8)
    {
        // TODO :Mem Erite
    }

    fn push_stack(&mut self, data: u8) {
        let address: u16 = 0x0100u16.wrapping_add(self.reg_sp as u16);
        self.write(address, data);
        self.reg_sp -= 1;
    }

    fn pop_stack(&mut self) -> u8 {
        self.reg_sp += 1;
        let address: u16 = 0x0100u16.wrapping_add(self.reg_sp as u16);
        self.read(address)
    }

    fn fetch_instruction(&mut self) -> u8 {
        let op_code = self.read(self.reg_pc);
        op_code
    }

    fn decode_instruction(&mut self, op_code: u8) {
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
            0xCB => { self.op_code = OpCode::PrefixCb; }, // Prefix CB
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

    fn decode_cb_instruction(&mut self, op_code: u8)
    {
        // TODO :CB拡張命令
    }

    fn execute_instruction(&mut self)
    {
        // TODO :CPU OP Exec
    }

    fn execute_cb_instruction(&mut self)
    {
        // TODO :CB拡張命令 Exec
    }
}

static mut S_CPU: Lazy<Pin<Box<LR35902>>> = Lazy::new(|| {
    let cpu = Box::pin(LR35902::new());
    cpu
});

fn cpu_reg_show()
{
    unsafe {
        let cpu = Pin::into_inner_unchecked(Pin::clone(&*S_CPU));
        println!("[DEBUG]: AF:{:#04X},BC:{:#04X},DE:{:#04X},HL:{:#04X},SP:{:#04X},PC:{:#04X}",
        cpu.reg_af, cpu.reg_bc, cpu.reg_de, cpu.reg_hl, cpu.reg_sp, cpu.reg_pc);
    }
}

fn cpu_proc()
{
    unsafe {
        let val = S_CPU.fetch_instruction();
        if S_CPU.cb != true {
            S_CPU.decode_instruction(val);
            S_CPU.execute_instruction();
        }else{
            S_CPU.decode_cb_instruction(val);
            S_CPU.execute_cb_instruction();
        }
    }
}

pub fn cpu_reset() -> Box<LR35902>
{
    unsafe {
        // TODO :CPU Reset
        let cpu_box: Box<LR35902> = Box::from_raw(Pin::as_mut(&mut *S_CPU).get_mut());
        cpu_box
    }
}

pub fn cpu_main()
{
    cpu_reg_show();
    cpu_proc();
}

// ====================================== TEST ======================================
#[cfg(test)]
mod cpu_test {
    use super::*;

    #[test]
    fn test_flags() {
        let mut cpu = LR35902::new();

        // フラグのクリアとセットのテスト
        cpu.set_status_flg(ZERO_FLG);
        assert_eq!(cpu.get_status_flg(ZERO_FLG), true);

        cpu.set_status_flg(NEGATIVE_FLG);
        assert_eq!(cpu.get_status_flg(NEGATIVE_FLG), true);
        cpu.set_status_flg(HALF_CARRY_FLG);
        assert_eq!(cpu.get_status_flg(HALF_CARRY_FLG), true);
        cpu.set_status_flg(CARRY_FLG);
        assert_eq!(cpu.get_status_flg(CARRY_FLG), true);

        // フラグのクリアのテスト
        cpu.cls_status_flg(NEGATIVE_FLG);
        assert_eq!(cpu.get_status_flg(NEGATIVE_FLG), false);
        cpu.cls_status_flg(HALF_CARRY_FLG);
        assert_eq!(cpu.get_status_flg(HALF_CARRY_FLG), false);
        cpu.cls_status_flg(CARRY_FLG);
        assert_eq!(cpu.get_status_flg(CARRY_FLG), false);
}

    // #[test]
    // fn cpu_test() {
    //     // TODO: 他のCPUテスト
    // }
}
// ==================================================================================