mod code_section;
mod function_section;
mod type_section;

use crate::decoder::{
    CodeSlice, DecoderError,
    sections::{
        code_section::CodeSection, function_section::FunctionSection, type_section::TypeSection,
    },
};

#[derive(Debug, Clone)]
pub enum Section {
    Type(TypeSection),
    Import,
    Function(FunctionSection),
    Memory,
    Export,
    Code(CodeSection),
    Data,
}

impl Section {
    pub fn decode(wasm: &mut CodeSlice) -> Result<Self, DecoderError> {
        let shdr = wasm.try_next_with_size(2)?.slice();
        let scode: u8 = shdr[0];
        let ssize: u8 = shdr[1];

        match scode {
            0x01 => Ok(Self::Type(TypeSection::decode(wasm)?)),
            // 0x02 => Ok(Self::Import(ImportSection::decode(wasm)?)),
            0x03 => Ok(Self::Function(FunctionSection::decode(wasm)?)),
            // 0x05 => Ok(Self::Memory(MemorySection::decode(wasm)?)),
            // 0x07 => Ok(Self::Export(ExportSection::decode(wasm)?)),
            0x0a => Ok(Self::Code(CodeSection::decode(wasm)?)),
            // 0x0b => Ok(Self::Data(DataSection::decode(wasm)?)),
            _ => Err(DecoderError::InvalidSectionCode),
        }
    }
}
