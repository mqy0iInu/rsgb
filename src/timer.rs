use common::IODevice;

const DIV_ADDR: u16 = 0xFF04;
const TIMA_ADDR: u16 = 0xFF05;
const TMA_ADDR: u16 = 0xFF06;
const TAC_ADDR: u16 = 0xFF07;

pub struct Timer {
    tima: u8,       // TIMA (Timer Counter)
    tma: u8,        // TMA (Timer Modulo)
    tac: u8,        // TAC (Timer control)
    cnt: u16,       // 16bit カウント値
    pub irq: bool,  // IRQ
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            tima: 0,
            tma: 0,
            tac: 0,
            cnt: 0,
            irq: false,
        }
    }
}

impl IODevice for Timer {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            DIV_ADDR => self.cnt = 0,
            TIMA_ADDR => self.tima = val,
            TMA_ADDR => self.tma = val,
            TAC_ADDR => self.tac = val & 0x7,
            _ => panic!("[ERR] Timer Write, Addr: 0x{:04x}", addr),
        }
    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            DIV_ADDR => (self.cnt >> 8) as u8,
            TIMA_ADDR => self.tima,
            TMA_ADDR => self.tma,
            TAC_ADDR => self.tac,
            _ => panic!("[ERR] Timer Read, Addr: 0x{:04x}", addr),
        }
    }

    fn update(&mut self, tick: u8) {
        let counter_prev = self.cnt;

        self.cnt = self.cnt.wrapping_add(tick as u16);

        if self.tac & 4 > 0 {
            let divider = match self.tac & 3 {
                0 => 10,
                1 => 4,
                2 => 6,
                3 | _ => 8,
            };

            let x = self.cnt >> divider;
            let y = counter_prev >> divider;
            let mask = (1 << (16 - divider)) - 1;
            let diff = x.wrapping_sub(y) & mask;

            if diff > 0 {
                let (res, overflow) = self.tima.overflowing_add(diff as u8);

                if overflow {
                    self.tima = self.tma + (diff as u8 - 1);
                    self.irq = true;
                } else {
                    self.tima = res;
                }
            }
        }
    }
}
