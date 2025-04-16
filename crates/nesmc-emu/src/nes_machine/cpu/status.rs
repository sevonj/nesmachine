use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct CpuStatus {
    /// Carry
    pub c: bool,
    /// Zero
    pub z: bool,
    /// Interrupt disable
    pub i: bool,
    /// Decimal (unused)
    pub d: bool,
    /// Overflow
    pub v: bool,
    /// Negative
    pub n: bool,
}

impl Default for CpuStatus {
    fn default() -> Self {
        Self {
            c: false,
            z: false,
            i: true,
            d: false,
            v: false,
            n: false,
        }
    }
}

impl Display for CpuStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}",
            if self.n { "N" } else { "-" },
            if self.v { "V" } else { "-" },
            if self.d { "D" } else { "-" },
            if self.i { "I" } else { "-" },
            if self.z { "Z" } else { "-" },
            if self.c { "C" } else { "-" },
        )
    }
}

impl From<u8> for CpuStatus {
    fn from(value: u8) -> Self {
        Self {
            c: value & 0x01 != 0,
            z: value & 0x02 != 0,
            i: value & 0x04 != 0,
            d: value & 0x08 != 0,

            v: value & 0x40 != 0,
            n: value & 0x81 != 0,
        }
    }
}

impl From<CpuStatus> for u8 {
    fn from(status: CpuStatus) -> Self {
        let mut value = 0x20;
        if status.c {
            value |= 0x1
        }
        if status.z {
            value |= 0x2
        }
        if status.i {
            value |= 0x4
        }
        if status.d {
            value |= 0x8
        }
        if status.v {
            value |= 0x40
        }
        if status.n {
            value |= 0x80
        }

        value
    }
}

impl CpuStatus {
    /// Reset button behavior
    pub fn reset(&mut self) {
        self.i = true
    }
}
