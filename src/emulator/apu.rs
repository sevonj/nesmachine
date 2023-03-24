pub(crate) struct APU {
    sq1_vol: u8,
    sq1_sweep: u8,
    sq1_lo: u8,
    sq1_hi: u8,
    sq2_vol: u8,
    sq2_sweep: u8,
    sq2_lo: u8,
    sq2_hi: u8,
    tri_linear: u8,
    tri_unused: u8,
    tri_lo: u8,
    tri_hi: u8,
    noise_vol: u8,
    noise_unused: u8,
    noise_lo: u8,
    noise_hi: u8,
    dmc_freq: u8,
    dmc_raw: u8,
    dmc_start: u8,
    dmc_len: u8,

    snd_chn: u8,
}

impl APU {
    pub(crate) fn new() -> Self {
        APU {
            sq1_vol: 0,
            sq1_sweep: 0,
            sq1_lo: 0,
            sq1_hi: 0,
            sq2_vol: 0,
            sq2_sweep: 0,
            sq2_lo: 0,
            sq2_hi: 0,
            tri_linear: 0,
            tri_unused: 0,
            tri_lo: 0,
            tri_hi: 0,
            noise_vol: 0,
            noise_unused: 0,
            noise_lo: 0,
            noise_hi: 0,
            dmc_freq: 0,
            dmc_raw: 0,
            dmc_start: 0,
            dmc_len: 0,
            snd_chn: 0,
        }
    }

    pub(crate) fn read(&self, address: u16) -> u8 {
        match address {
            // read from apu registers
            // ...
            _ => 0, // return 0 for all other addresses
        }
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        match address {
            // write to apu registers
            // ...
            _ => (), // do nothing for all other addresses
        }
    }
}
