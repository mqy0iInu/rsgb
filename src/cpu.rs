const ZERO_FLG: u8 = 0b1000_0000;       // bit7: Z Flag.
const NEGATIVE_FLG: u8 = 0b0100_0000;   // bit6: N Flag.
const HALF_CARRY_FLG: u8 = 0b0010_0000; // bit5: H Flag.
const CARRY_FLG: u8 = 0b0001_0000;      // bit4: C Flag.

const BOOT_ROM_ADDR: u16 = 0x0000;

#[derive(Clone)]
pub enum OpCode {
    // Load/Store Operations
    LD, LDI, LDD,
    // Arithmetic/Logical Operations
    ADD, AND, SUB, OR, XOR, ADC, SBC, CP, INC, DEC, DAA, CPL,
    // Shift and Rotate Operations
    RLCA, RLA, RRCA, RRA, RLC, RL, RRC, RR, SLA, SWAP, SRA, SRL,
    // Jump and Call Operations
    JP, JR, CALL, RET, RETI, RST,
    // Bit Operations
    BIT, SET, RES,
    // Other
    NOP, CCF, SCF, HALT, STOP, DI, EI,
    // Undefined OP
    UNK,
}

#[derive(Clone)]
pub enum Addressing {
    IMM,        // Immediate value
    REG,        // Register (A, B, C, D, E, H, L)
    IND,        // Indirect (HL)
    IND_INC,    // Indirect with auto-increment (HL+)
    IND_DEC,    // Indirect with auto-decrement (HL-)
    INDX,       // Indexed with immediate value (e.g., LD A, (nn))
    ABS,        // Absolute (nn)
    ABS_IDX,    // Absolute indexed (e.g., LD A, (HL))
    REL,        // Relative (PC + n)
    BIT,        // Bit addressing (e.g., BIT b, r)
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