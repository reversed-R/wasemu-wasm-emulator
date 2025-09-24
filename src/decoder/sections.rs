pub(crate) mod code_section;
pub(crate) mod function_section;
pub(crate) mod type_section;

use crate::decoder::{ DecoderError, RemainCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SectionId {
    Custom = 0x00,
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Memory = 0x05,
    Export = 0x07,
    Code = 0x0a,
    Data = 0x0b,
}

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub id: SectionId,
    pub size: u32,
}

impl SectionHeader {
    pub fn decode<'code>(
        wasm: RemainCode<'code>,
    ) -> Result<(Self, Option<RemainCode<'code>>), DecoderError> {
        let (id, opt_wasm) = wasm.try_next_byte()?;

        // size of section (bytes)
        let (size, opt_wasm) = opt_wasm
            .ok_or(DecoderError::InvalidCodeSize)?
            .try_next_leb128()?;

        Ok((
            Self {
                id: SectionId::try_from(id)?,
                size,
            },
            opt_wasm,
        ))
    }
}

impl SectionId {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Custom => 0x00,
            Self::Type => 0x01,
            Self::Import => 0x02,
            Self::Function => 0x03,
            Self::Memory => 0x05,
            Self::Export => 0x07,
            Self::Code => 0x0a,
            Self::Data => 0x0b,
        }
    }
}

impl TryFrom<u8> for SectionId {
    type Error = DecoderError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Custom),
            0x01 => Ok(Self::Type),
            0x02 => Ok(Self::Import),
            0x03 => Ok(Self::Function),
            0x05 => Ok(Self::Memory),
            0x07 => Ok(Self::Export),
            0x0a => Ok(Self::Code),
            0x0b => Ok(Self::Data),
            _ => Err(DecoderError::InvalidSectionCode),
        }
    }
}
