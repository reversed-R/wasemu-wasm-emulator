use crate::decoder::{DecoderError, jit::JITDecoder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    End,
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    I32Add,
}

impl Instr {
    pub(super) fn decode(decoder: &mut JITDecoder) -> Result<Self, DecoderError> {
        match decoder.try_next_byte()? {
            0x0b => Ok(Self::End),
            0x20 => Ok(Self::LocalGet(decoder.try_next_leb128()?)),
            0x21 => Ok(Self::LocalSet(decoder.try_next_leb128()?)),
            0x22 => Ok(Self::LocalTee(decoder.try_next_leb128()?)),
            0x6a => Ok(Self::I32Add),
            _ => Err(DecoderError::InvalidInstructionCode),
        }
    }
}
