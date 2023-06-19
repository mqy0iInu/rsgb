use crate::instruction::*;
use std::pin::Pin;
use once_cell::sync::Lazy;

const ZERO_FLG: u8 = 0b1000_0000;      // bit7: Z Flag.
const NEGATIVE_FLG: u8 = 0b0100_0000;  // bit6: N Flag.
const HALF_CARRY_FLG: u8 = 0b0010_0000; // bit5: H Flag.
const CARRY_FLG: u8 = 0b0001_0000;     // bit4: C Flag.

const BOOT_ROM_ADDR: u16 = 0x0000;

pub const REG_AF: u8 = 0;
pub const REG_BC: u8 = 1;
pub const REG_DE: u8 = 2;
pub const REG_HL: u8 = 3;
pub const REG_SP: u8 = 4;
pub const REG_PC: u8 = 5;

#[derive(Clone)]
pub struct LR35902
{
    pub reg_af: u16, // AF
    pub reg_bc: u16, // BC
    pub reg_de: u16, // DE
    pub reg_hl: u16, // HL
    pub reg_sp: u16,
    pub reg_pc: u16,

    pub inst: Instruction,

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

            inst: Instruction::new(),

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
        let val = S_CPU.inst.fetch_instruction();
        S_CPU.inst.decode_instruction(val);
        if S_CPU.inst.cb != true {
            S_CPU.inst.execute_instruction();
        }else{
            S_CPU.inst.execute_cb_instruction();
        }
    }
}

pub fn cpu_mem_read(addr: u16) -> u8
{
    unsafe {
        S_CPU.read(addr)
    }
}

pub fn cpu_mem_write(addr: u16, val: u8)
{
    unsafe {
        S_CPU.write(addr, val);
    }
}
pub fn get_cpu_reg(reg: u8) -> u16
{
    unsafe {
        match reg {
            REG_AF => S_CPU.reg_af,
            REG_BC => S_CPU.reg_bc,
            REG_DE => S_CPU.reg_de,
            REG_HL => S_CPU.reg_hl,
            REG_SP => S_CPU.reg_sp,
            REG_PC | _ => S_CPU.reg_pc
        }
    }
}

pub fn set_cpu_reg(reg: u8, val: u16)
{
    unsafe {
        match reg {
            REG_AF => S_CPU.reg_af = val,
            REG_BC => S_CPU.reg_bc = val,
            REG_DE => S_CPU.reg_de = val,
            REG_HL => S_CPU.reg_hl = val,
            REG_SP => S_CPU.reg_sp = val,
            REG_PC | _ => S_CPU.reg_pc = val,
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