use color_eyre::{
    eyre::{bail, eyre},
    Result,
};

#[derive(Debug)]
pub struct Memory([u8; 4 * 1024]);

impl Memory {
    pub const FONT_OFFSET: u16 = 0x50;
    pub const ROM_OFFSET: u16 = 512;

    pub fn new(rom: &[u8]) -> Self {
        // 4 KiB
        let mut memory = [0; 4 * 1024];

        // font data
        // 0x50 appears to be a popular place to put it
        memory[Self::FONT_OFFSET as usize..(Self::FONT_OFFSET + 0x50) as usize].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);

        // apparently some roms expect to be place at offset 512 in memory
        memory[Self::ROM_OFFSET as usize..Self::ROM_OFFSET as usize + rom.len()]
            .copy_from_slice(rom);

        Self(memory)
    }

    pub fn read_u8(&self, pos: u16) -> Result<u8> {
        self.0
            .get(pos as usize)
            .ok_or_else(|| eyre!("tried to read out of bounds: {pos} > {}", self.0.len()))
            .copied()
    }

    pub fn write_u8(&mut self, pos: u16, byte: u8) -> Result<()> {
        let pos = pos as usize;

        if pos > self.0.len() {
            bail!("tried to write out of bounds: {pos} > {}", self.0.len())
        }

        self.0[pos] = byte;
        Ok(())
    }

    pub fn read_u16(&self, pos: u16) -> Result<u16> {
        let byte1 = self.read_u8(pos)? as u16;
        let byte2 = self.read_u8(pos + 1)? as u16;

        Ok((byte1 << 8) | byte2)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; 4 * 1024])
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Registers([u8; 16]);

impl Registers {
    pub const fn new() -> Self {
        Self([0; 16])
    }
}

impl std::ops::Index<RegIdx> for Registers {
    type Output = u8;

    fn index(&self, index: RegIdx) -> &Self::Output {
        &self.0[index.as_u8() as usize]
    }
}

impl std::ops::IndexMut<RegIdx> for Registers {
    fn index_mut(&mut self, index: RegIdx) -> &mut Self::Output {
        &mut self.0[index.as_u8() as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RegIdx(u8);

impl RegIdx {
    pub const FLAG: RegIdx = RegIdx(0xF);

    pub const fn new(idx: u8) -> Self {
        if idx > 15 {
            panic!("registed idx > 15")
        }

        Self(idx)
    }

    pub const fn as_u8(&self) -> u8 {
        self.0
    }
}

impl std::fmt::Display for RegIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V{}", self.0)
    }
}
