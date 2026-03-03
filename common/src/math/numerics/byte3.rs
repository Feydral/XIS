#![allow(dead_code)]

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Byte3 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

impl Byte3 {
    pub const fn new(a: u8, b: u8, c: u8) -> Byte3 {
        Byte3 { a, b, c }
    }

    pub const fn from_u32(value: u32) -> Byte3 {
        Byte3 {
            a: ((value >> 16) & 0xFF) as u8,
            b: ((value >> 8) & 0xFF) as u8,
            c: (value & 0xFF) as u8,
        }
    }

    pub fn modify(&mut self, index: usize, value: u8) -> &mut Self {
        let value = (value != 0) as u8;

        let (byte, bit) = match index {
            0..=7 => (&mut self.c, index),
            8..=15 => (&mut self.b, index - 8),
            16..=23 => (&mut self.a, index - 16),
            _ => panic!("Bit index out of range for Byte3: {}", index),
        };

        if value == 1 {
            *byte |= 1 << bit;
        } else {
            *byte &= !(1 << bit);
        }

        self
    }
}