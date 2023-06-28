use common::IODevice;

pub struct GamePad {
    key: u8,
    key_state: u8,
    pub irq: bool,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Key {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
}

impl GamePad {
    pub fn new() -> Self {
        GamePad {
            key: 0xff,
            key_state: 0xff,
            irq: false,
        }
    }

    pub fn keydown(&mut self, key: Key) {
        match key {
            Key::Down => self.key_state &= !0x80,
            Key::Up => self.key_state &= !0x40,
            Key::Left => self.key_state &= !0x20,
            Key::Right => self.key_state &= !0x10,
            Key::Start => self.key_state &= !0x08,
            Key::Select => self.key_state &= !0x04,
            Key::B => self.key_state &= !0x02,
            Key::A => self.key_state &= !0x01,
        }

        self.irq = true;
    }

    pub fn keyup(&mut self, key: Key) {
        match key {
            Key::Down => self.key_state |= 0x80,
            Key::Up => self.key_state |= 0x40,
            Key::Left => self.key_state |= 0x20,
            Key::Right => self.key_state |= 0x10,
            Key::Start => self.key_state |= 0x08,
            Key::Select => self.key_state |= 0x04,
            Key::B => self.key_state |= 0x02,
            Key::A => self.key_state |= 0x01,
        }
    }
}

impl IODevice for GamePad {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF00 => self.key = (self.key & 0xcf) | (val & 0x30),
            _ => unreachable!("Unexpected address: 0x{:04x}", addr),
        }
    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => {
                // Direction keys selected
                if self.key & 0x10 == 0 {
                    (self.key & 0xF0) | (self.key_state >> 4) & 0x0F
                // Button keys selected
                } else if self.key & 0x20 == 0 {
                    (self.key & 0xF0) | self.key_state & 0x0F
                } else {
                    self.key
                }
            }
            _ => unreachable!("Unexpected address: 0x{:04x}", addr),
        }
    }

    fn update(&mut self, _tick: u8) {}
}
