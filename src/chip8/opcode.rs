use std::fmt;

/// Opcode is always 2 bytes in CHIP8

pub struct Opcode {
    value: u16,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06x}", self.value)
    }
}

impl Opcode {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    /// Returns the value of the upcode as u16
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Returns the value of the upcode as tuple of 4 bits
    pub fn per_4bits(&self) -> (usize, usize, usize, usize) {
        let v = self.value as usize;
        (
            (v & 0xF000) >> 12,
            (v & 0x0F00) >> 8,
            (v & 0x00F0) >> 4,
            v & 0x000F,
        )
    }

    pub fn nnn(&self) -> u16 {
        0x0FFF & self.value
    }

    pub fn nn(&self) -> u8 {
        let v = self.value & 0xFF;
        v.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_4bits() {
        let opcode = Opcode::new(0xF123);
        assert_eq!(opcode.per_4bits(), (0xF, 0x1, 0x2, 0x3));

        let opcode = Opcode::new(0x23);
        assert_eq!(opcode.per_4bits(), (0x0, 0x0, 0x2, 0x3));

        let opcode = Opcode::new(0x23F);
        assert_eq!(opcode.per_4bits(), (0x0, 0x2, 0x3, 0xF));
    }

    #[test]
    fn test_nnn() {
        let opcode = Opcode::new(0xF123);
        assert_eq!(opcode.nnn(), 0x0123);
    }

    #[test]
    fn test_nn() {
        let opcode = Opcode::new(0xF123);
        assert_eq!(opcode.nn(), 0x23);
    }
}
