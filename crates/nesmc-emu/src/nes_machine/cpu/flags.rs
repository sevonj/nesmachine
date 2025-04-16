use super::Cpu;

impl Cpu {
    pub(super) fn set_negative(&mut self, result: u8) {
        self.p.n = result & 0x80 != 0;
    }

    pub(super) fn set_overflow(&mut self, data: u8, result: u8) {
        let sign_a = self.a & 0x80 != 0;
        let sign_b = data & 0x80 != 0;
        let sign_res = result & 0x80 != 0;

        self.p.v = (sign_a && sign_b && !sign_res) || (!sign_a && !sign_b && sign_res);
    }

    pub(super) fn set_carry(&mut self, yes: bool) {
        self.p.c = yes;
    }

    pub(super) fn set_int_disable(&mut self, yes: bool) {
        self.p.i = yes;
    }

    pub(super) fn set_zero(&mut self, result: u8) {
        self.p.z = result == 0;
    }
}
