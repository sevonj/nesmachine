use super::Cpu;

impl Cpu {
    pub(super) fn set_negative(&mut self, value: u8) {
        self.status.n = value & 0x80 != 0;
    }

    pub(super) fn set_overflow(&mut self, value: u8, result: u8) {
        let sign_a = self.a & 0x80 != 0;
        let sign_b = value & 0x80 != 0;
        let sign_res = result & 0x80 != 0;

        self.status.v = (sign_a && sign_b && !sign_res) || (!sign_a && !sign_b && sign_res);
    }

    pub(super) fn set_zero(&mut self, value: u8) {
        self.status.z = value == 0;
    }
}
