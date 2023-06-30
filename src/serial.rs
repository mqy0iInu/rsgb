use common::*;

const _CLOCK_SPEED_NORMAL: u8 = 0;
const _CLOCK_SPEED_FAST: u8 = 1;

const _SHIFT_CLOCK_INTERNAL: u8 = 0;
const _SHIFT_CLOCK_EXTERNAL: u8 = 1;

// TODO SPIの実装(Master, Slave)
// https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html#serial-data-transfer-link-cable
#[allow(dead_code)]
pub struct Serial {
    sb: u8,               // SB(Serial転送データレジスタ)
    sc: u8,               // SC(Serial転送制御レジスタ)
    tick: u8,             // SPI クロック
    pub irq: bool,        // IRQ

    // SC Bit
    is_start_req: bool,  // SC Bit7: 転送要求フラグ
    clock_speed: u8,     // SC Bit1: (※CGB Only) 転送スピード(0 = Normal, 1 = Fast)
    shift_clock: u8,     // SC Bit0: Shift Clock (0 = 外部クロック, 1 = 内部クロック)
}

#[allow(dead_code)]
impl Serial {
    pub fn new() -> Self {
        Serial {
            sb: 0,
            sc: 0,
            tick: 0,
            irq: false,

            is_start_req: false,
            clock_speed: _CLOCK_SPEED_NORMAL,
            shift_clock: _SHIFT_CLOCK_INTERNAL,
        }
    }

    fn clock_speed_change(&mut self) {
        // TODO :SPI IRQ
        self.irq = true;
    }

    fn spi_tx(&mut self, _val: u8) {
        // TODO :SPI TX
    }

    fn spi_rx(&self) -> u8{
        // TODO :SPI RX
        0xFF
    }

    fn spi_irq(&mut self) {
        // TODO :SPI IRQ
        self.irq = true;
    }

    // SPI 割込みハンドラ
    // https://gbdev.io/pandocs/Interrupt_Sources.html#int-58--serial-interrupt
    fn spi_irq_handler(&mut self) {
        self.spi_tx(self.sb);
        self.irq = false;
        // TODO $0058にJMPする???
    }
}

#[allow(dead_code)]
impl IO for Serial {
    fn write(&mut self, addr: u16, _val: u8) {
        match addr {
            0xFF01 => self.sb = _val,
            0xFF02 => self.sc = _val,
            _ => panic!("[ERR] Serial Write Only! (Addr: ${:#04X})", addr),
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            _ => panic!("[ERR] Serial Read Addr ${:#04X}", addr),
        }
    }

    fn update(&mut self, _tick: u8) {
        // SC(Serial転送制御レジスタ)
        self.is_start_req = (self.sc & _BIT_7) != 0;
        self.clock_speed = (self.sc & _BIT_1) >> 1;
        self.shift_clock = self.sc & _BIT_0;

        // IRQ
        if self.is_start_req != false {
            self.spi_irq();
        }
    }
}
