mod code_section;
mod function_section;
mod type_section;

use crate::decoder::{
    DecoderError, RemainCode,
    sections::{
        code_section::JITCodeSection, function_section::FunctionSection, type_section::TypeSection,
    },
};

#[derive(Debug, Clone)]
pub enum Section<'code> {
    Type(TypeSection),
    Import,
    Function(FunctionSection),
    Memory,
    Export,
    Code(JITCodeSection<'code>),
    Data,
}

impl<'code> Section<'code> {
    pub fn decode(
        wasm: RemainCode<'code>,
    ) -> Result<(Self, Option<RemainCode<'code>>), DecoderError> {
        let (scode, opt_wasm) = wasm.try_next_byte()?;

        // size of section (bytes)
        let (ssize, opt_wasm) = opt_wasm
            .ok_or(DecoderError::InvalidCodeSize)?
            .try_next_leb128()?;
        let wasm = opt_wasm.ok_or(DecoderError::InvalidCodeSize)?;

        match scode {
            0x01 => {
                let (type_section, wasm) = TypeSection::decode(wasm)?;

                Ok((Self::Type(type_section), wasm))
            }
            // 0x02 => Ok(Self::Import(ImportSection::decode(wasm)?)),
            0x03 => {
                let (func_section, wasm) = FunctionSection::decode(wasm)?;

                Ok((Self::Function(func_section), wasm))
            }
            // 0x05 => Ok(Self::Memory(MemorySection::decode(wasm)?)),
            // 0x07 => Ok(Self::Export(ExportSection::decode(wasm)?)),
            0x0a => {
                let (code_section, wasm) = JITCodeSection::decode(wasm, ssize as usize)?;

                Ok((Self::Code(code_section), wasm))
            }
            // 0x0b => Ok(Self::Data(DataSection::decode(wasm)?)),
            _ => Err(DecoderError::InvalidSectionCode),
        }
    }
}
