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
}