use super::Bus;
impl Bus {
    pub fn load_rom(&mut self, filepath: &str) {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(filepath).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        let header = &buffer[0..16];
        let mapper = ((header[6] & 0xF0) >> 4) | (header[7] & 0xF0);
        self.mapper = mapper as u32;

        let prg_rom_size = header[4] as usize * 0x4000;
        let prg_rom_start = 16;
        let prg_rom_end = prg_rom_start + prg_rom_size;
        let prg_rom = &buffer[prg_rom_start..prg_rom_end];
        self.prg_rom = vec![0; prg_rom_size];
        self.prg_rom[..prg_rom_size].clone_from_slice(prg_rom);

        println!("Loaded: PRG_ROM size: {:04x}", prg_rom_size);

        println!(
            "chr first 8 before loading: {}, {}, {}, {}, {}, {}, {}, {}, ",
            self.ppu.chr_rom[0x20],
            self.ppu.chr_rom[0x21],
            self.ppu.chr_rom[0x22],
            self.ppu.chr_rom[0x23],
            self.ppu.chr_rom[0x24],
            self.ppu.chr_rom[0x25],
            self.ppu.chr_rom[0x26],
            self.ppu.chr_rom[0x27]
        );
        let chr_rom_size = header[5] as usize * 0x2000;
        let chr_rom_start = prg_rom_end;
        let chr_rom_end = chr_rom_start + chr_rom_size;
        let chr_rom = &buffer[chr_rom_start..chr_rom_end];
        self.ppu.chr_rom = vec![0; chr_rom_size];
        self.ppu.chr_rom[..chr_rom_size].clone_from_slice(chr_rom);

        println!(
            "chr first 8 after loading: {}, {}, {}, {}, {}, {}, {}, {}, ",
            self.ppu.chr_rom[0x20],
            self.ppu.chr_rom[0x21],
            self.ppu.chr_rom[0x22],
            self.ppu.chr_rom[0x23],
            self.ppu.chr_rom[0x24],
            self.ppu.chr_rom[0x25],
            self.ppu.chr_rom[0x26],
            self.ppu.chr_rom[0x27]
        );

        println!("prg rom start: {:x} end: {:x}", prg_rom_start, prg_rom_end);
        println!("chr rom start: {:x} end: {:x}", chr_rom_start, chr_rom_end);

        println!("Loaded: CHR_ROM size: {:04x}", chr_rom_size);
    }
}
