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

    pub fn value(&self) -> u16 {
        self.value
    }

    /// Returns the upper 4 bits of the opcode
    pub fn upper4(&self) -> u8 {
        let upper = self.value >> 12;
        upper.try_into().unwrap()
    }

    #[allow(dead_code)]
    pub fn nnn(&self) -> u16 {
        0x0FFF & self.value
    }

    #[allow(dead_code)]
    pub fn nn(&self) -> u8 {
        let v = self.value & 0xFF;
        v.try_into().unwrap()
    }

    #[allow(dead_code)]
    pub fn n(&self) -> u8 {
        let v = self.value & 0xF;
        v.try_into().unwrap()
    }

    #[allow(dead_code)]
    pub fn x(&self) -> u8 {
        let v = (self.value & 0x0F00) >> 8;
        v.try_into().unwrap()
    }

    #[allow(dead_code)]
    pub fn y(&self) -> u8 {
        let v = (self.value & 0x00F0) >> 4;
        v.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper4() {
        let opcode = Opcode::new(0xF123);
        assert_eq!(opcode.upper4(), 0xF);

        let opcode = Opcode::new(0x4123);
        assert_eq!(opcode.upper4(), 0x4);

        let opcode = Opcode::new(0x0123);
        assert_eq!(opcode.upper4(), 0x0);
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

    #[test]
    fn test_n() {
        let opcode = Opcode::new(0xF123);
        assert_eq!(opcode.n(), 0x3);
    }

    #[test]
    fn test_x() {
        let opcode = Opcode::new(0xF123);
        assert_eq!(opcode.x(), 0x1);
    }

    #[test]
    fn test_y() {
        let opcode = Opcode::new(0xF123);
        assert_eq!(opcode.y(), 0x2);
    }
}
