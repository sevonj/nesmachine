use super::MapperIo;

const KIB: usize = 1024;

#[derive(Debug)]
enum NametableArrangement {
    OneScreenLower,
    OneScreenUpper,
    /// Vertical mirroring
    HorizontalArrangement,
    /// Horizontal mirroring
    VerticalArrangement,
}

impl NametableArrangement {
    pub fn from_sr_byte(value: u8) -> Self {
        match value | 0x03 {
            0 => Self::OneScreenLower,
            1 => Self::OneScreenUpper,
            2 => Self::HorizontalArrangement,
            3 => Self::VerticalArrangement,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum PrgBankMode {
    /// 1x32KB. Switched. Ignores low bit of bank no.
    Big,
    /// 2x16KB. First is fixed, second is switched.
    SplitFixFirst,
    /// 2x16KB. First is switched, second is fixed.
    SplitFixLast,
}

impl PrgBankMode {
    pub fn from_sr_byte(value: u8) -> Self {
        match (value >> 2) | 0x03 {
            0 | 1 => Self::Big,
            2 => Self::SplitFixFirst,
            3 => Self::SplitFixLast,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum ChrBankMode {
    /// 1x8KB. Switched.
    Big,
    /// 2x4KB. Switched.
    Split,
}

impl ChrBankMode {
    pub fn from_sr_byte(value: u8) -> Self {
        if value & (1 << 5) != 0 {
            Self::Big
        } else {
            Self::Split
        }
    }
}

#[derive(Debug)]
pub struct Mmc1 {
    /// Optional 8 KB program ram
    prg_ram: Option<Vec<u8>>,
    prg_rom: Vec<u8>,
    _chr_rom: Vec<u8>,

    /// Shift register
    sr: u8,
    sr_write_counter: u8,

    arrangement: NametableArrangement,
    prg_mode: PrgBankMode,
    prg_bank_offset: usize,
    chr_mode: ChrBankMode,
}

impl Mmc1 {
    const SR_RESET_BIT: u8 = 0x80;

    pub fn new(prg_ram: Option<Vec<u8>>, prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Self {
        Self {
            prg_ram,
            prg_rom,
            _chr_rom: chr_rom,

            sr: 0,
            sr_write_counter: 0,

            // TODO: Maybe right, maybe not
            arrangement: NametableArrangement::OneScreenLower,
            // TODO: Maybe right, maybe not
            prg_mode: PrgBankMode::Big,
            // TODO: Start at last bank?
            prg_bank_offset: 0,
            // TODO: Maybe right, maybe not
            chr_mode: ChrBankMode::Big,
        }
    }

    /// CPU RAM $6000-$7fff
    fn read_prg_ram(&self, addr: u16) -> u8 {
        let Some(prg_ram) = &self.prg_ram else {
            return 0;
        };
        prg_ram[addr as usize]
    }

    /// CPU ROM $8000-$ffff
    fn read_prg_rom(&self, addr: u16) -> u8 {
        let local_addr = (addr - 0x8000) as usize;

        let off_in_bank = match self.prg_mode {
            PrgBankMode::Big => local_addr % (32 * KIB),
            PrgBankMode::SplitFixFirst | PrgBankMode::SplitFixLast => local_addr % (16 * KIB),
        };

        let mapped_addr = match self.prg_mode {
            PrgBankMode::Big => off_in_bank + self.prg_bank_offset,
            PrgBankMode::SplitFixFirst => match addr {
                0x8000..=0xbfff => off_in_bank,
                _ => off_in_bank + self.prg_bank_offset,
            },
            PrgBankMode::SplitFixLast => match addr {
                0xc000..=0xffff => off_in_bank,
                _ => off_in_bank + self.prg_bank_offset,
            },
        };

        self.prg_rom[mapped_addr % self.prg_rom.len()]
    }

    pub fn write_sr(&mut self, addr: u16, value: u8) {
        //       7      0
        // bits: R______D
        //
        // D: Data
        // R: Reset

        if value & Self::SR_RESET_BIT > 0 {
            self.sr_write_counter = 0;
            self.sr = 0;
        } else {
            self.sr_write_counter += 1;
            self.sr >>= 1;
            self.sr |= (value & 1) << 4;

            if self.sr_write_counter == 5 {
                match addr {
                    0x0000..=0x7fff => unreachable!(),
                    0x8000..=0x9fff => self.commit_control(),
                    0xa000..=0xbfff => self.commit_chr_0(),
                    0xc000..=0xdfff => self.commit_chr_1(),
                    0xe000..=0xffff => self.commit_prg(),
                }
                self.sr_write_counter = 0;
                self.sr = 0;
            }
        }
    }

    fn commit_control(&mut self) {
        //       4   0
        // bits: CPPMM
        //
        // C: CHR bank mode
        // P: PRG bank mode
        // M: Arrangement

        self.arrangement = NametableArrangement::from_sr_byte(self.sr);
        self.prg_mode = PrgBankMode::from_sr_byte(self.sr);
        self.chr_mode = ChrBankMode::from_sr_byte(self.sr);
    }

    fn commit_chr_0(&mut self) {
        //
    }

    fn commit_chr_1(&mut self) {
        //
    }

    fn commit_prg(&mut self) {
        //       4   0
        // bits: RPPPP
        //
        // R: (UNUSED in MMC1A) PRG-RAM enable TODO: Rest of the revs
        // P: PRG bank offset 16KB

        let mut bank_no = self.sr as usize & 0x0f;

        // 32KB mode ignores least significant bit, doesn't switch in 16KB steps.
        if let PrgBankMode::Big = self.prg_mode {
            bank_no &= 0xfe;
        }

        self.prg_bank_offset = bank_no * 16 * KIB;
    }
}

impl MapperIo for Mmc1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x5fff => 0,
            0x6000..=0x7fff => self.read_prg_ram(addr),
            0x8000..=0xffff => self.read_prg_rom(addr),
        }
    }

    fn write(&mut self, addr: u16, _value: u8) {
        match addr {
            0x0000..=0x7fff => (),
            0x8000..=0xffff => todo!(),
        }
    }
}
