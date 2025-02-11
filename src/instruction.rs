use crate::memory::RegIdx;

/// Describes CHIP-8 instructions
/// Each variant is commented with the opcode, where:
/// - X is the second nibble (used to index register VX)
/// - Y is the third nibble (used to index register VY)
/// - N is the fourth nubble (4-bit number)
/// - NN is the second byte (8-bit immediate number)
/// - NNN is nibbles 2, 3, 4 (12-bit immediate memory address)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// 00E0
    ClearScreen,
    /// 1NNN
    Jump(u16),
    /// 00EE
    Return,
    /// 2NNN
    Call(u16),
    /// 3XNN
    SkipEqIm(RegIdx, u8),
    /// 4XNN
    SkipNeIm(RegIdx, u8),
    /// 5XY0
    SkipEq(RegIdx, RegIdx),
    /// 9XY0
    SkipNe(RegIdx, RegIdx),
    /// 6XNN
    SetIm(RegIdx, u8),
    /// 7XNN
    AddIm(RegIdx, u8),
    /// 8XY0
    Set(RegIdx, RegIdx),
    /// 8XY1
    Or(RegIdx, RegIdx),
    /// 8XY2
    And(RegIdx, RegIdx),
    /// 8XY3
    Xor(RegIdx, RegIdx),
    /// 8XY4
    Add(RegIdx, RegIdx),
    /// 8XY5
    Sub(RegIdx, RegIdx),
    /// 8XY7
    SubOpp(RegIdx, RegIdx),
    /// 8XY6
    Shr(RegIdx, RegIdx),
    /// 8XYE
    Shl(RegIdx, RegIdx),
    /// ANNN
    SetIndex(u16),
    /// BNNN
    JumpV0(u16),
    /// CXNN
    RandAnd(RegIdx, u8),
    /// DXYN
    Draw(RegIdx, RegIdx, u8),
    /// EX9E
    SkipKeyEq(RegIdx),
    /// EXA1
    SkipKeyNe(RegIdx),
    /// FX07
    GetDelay(RegIdx),
    /// FX15
    SetDelay(RegIdx),
    /// FX18
    SetSound(RegIdx),
    /// FX1E
    AddIndex(RegIdx),
    /// FX0A
    GetKey(RegIdx),
    /// FX29
    IndexCharacter(RegIdx),
    /// FX33
    SetBcd(RegIdx),
    /// FX55
    RegStore(RegIdx),
    /// FX65
    RegLoad(RegIdx),
}

impl Instruction {
    pub fn decode(word: u16) -> Option<Self> {
        use Instruction::*;

        Some(match word.nibble1() {
            0 if word.byte2() == 0xE0 => ClearScreen,
            0 if word.byte2() == 0xEE => Return,
            1 => Jump(word.bits12()),
            2 => Call(word.bits12()),
            3 => SkipEqIm(RegIdx::new(word.nibble2()), word.byte2()),
            4 => SkipNeIm(RegIdx::new(word.nibble2()), word.byte2()),
            5 if word.nibble4() == 0 => {
                SkipEq(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            6 => SetIm(RegIdx::new(word.nibble2()), word.byte2()),
            7 => AddIm(RegIdx::new(word.nibble2()), word.byte2()),
            8 if word.nibble4() == 0 => {
                Set(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 1 => {
                Or(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 2 => {
                And(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 3 => {
                Xor(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 4 => {
                Add(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 5 => {
                Sub(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 6 => {
                Shr(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 7 => {
                SubOpp(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            8 if word.nibble4() == 0xE => {
                Shl(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            9 if word.nibble4() == 0 => {
                SkipNe(RegIdx::new(word.nibble2()), RegIdx::new(word.nibble3()))
            }
            0xA => SetIndex(word.bits12()),
            0xB => JumpV0(word.bits12()),
            0xC => RandAnd(RegIdx::new(word.nibble2()), word.byte2()),
            0xD => Draw(
                RegIdx::new(word.nibble2()),
                RegIdx::new(word.nibble3()),
                word.nibble4(),
            ),
            0xE if word.byte2() == 0x9E => SkipKeyEq(RegIdx::new(word.nibble2())),
            0xE if word.byte2() == 0xA1 => SkipKeyNe(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x07 => GetDelay(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x0A => GetKey(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x15 => SetDelay(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x18 => SetSound(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x1E => AddIndex(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x29 => IndexCharacter(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x33 => SetBcd(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x55 => RegStore(RegIdx::new(word.nibble2())),
            0xF if word.byte2() == 0x65 => RegLoad(RegIdx::new(word.nibble2())),

            _ => return None,
        })
    }
}

/// A simple  helper trait to extract data needed by instructions
pub trait IntExt {
    fn nibble1(&self) -> u8;
    fn nibble2(&self) -> u8;
    fn nibble3(&self) -> u8;
    fn nibble4(&self) -> u8;

    fn byte2(&self) -> u8;
    fn bits12(&self) -> u16;
}

impl IntExt for u16 {
    #[inline]
    fn nibble1(&self) -> u8 {
        (self >> 12) as u8
    }

    #[inline]
    fn nibble2(&self) -> u8 {
        ((self & 0x0F00) >> 8) as u8
    }

    #[inline]
    fn nibble3(&self) -> u8 {
        ((self & 0x00F0) >> 4) as u8
    }

    #[inline]
    fn nibble4(&self) -> u8 {
        (self & 0x000F) as u8
    }

    #[inline]
    fn byte2(&self) -> u8 {
        (self & 0x00FF) as u8
    }

    #[inline]
    fn bits12(&self) -> u16 {
        self & 0x0FFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_clear_screen() {
        assert_eq!(
            Instruction::decode(0x00E0).unwrap(),
            Instruction::ClearScreen
        );
    }

    #[test]
    fn decode_jump() {
        assert_eq!(
            Instruction::decode(0x1FFF).unwrap(),
            Instruction::Jump(4095)
        );
    }

    #[test]
    fn decode_set() {
        assert_eq!(
            Instruction::decode(0x61FF).unwrap(),
            Instruction::SetIm(RegIdx::new(1), 255)
        );
    }

    #[test]
    fn decode_add() {
        assert_eq!(
            Instruction::decode(0x7EAA).unwrap(),
            Instruction::AddIm(RegIdx::new(14), 170)
        );
    }

    #[test]
    fn decode_set_index() {
        assert_eq!(
            Instruction::decode(0xAAAA).unwrap(),
            Instruction::SetIndex(2730)
        );
    }

    #[test]
    fn decode_draw() {
        assert_eq!(
            Instruction::decode(0xDA1C).unwrap(),
            Instruction::Draw(RegIdx::new(10), RegIdx::new(1), 12)
        );
    }
}
