mod sections;

use crate::decoder::sections::Section;

#[derive(Debug, Clone, Copy)]
struct WasmVersion(u32);

#[derive(Debug, Clone)]
pub struct WasmModule {
    version: WasmVersion,
    sections: Vec<Section>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderError {
    InvalidCodeSize(usize),
    InvalidMagic,
    InvalidSectionCode,
}

impl WasmModule {
    pub fn decode(wasm: &[u8]) -> Result<Self, DecoderError> {
        let mut wasm = CodeSlice::try_new(wasm, 0, 8)?;

        if &wasm.slice()[..4] != "\0asm".as_bytes() {
            Err(DecoderError::InvalidMagic)
        } else {
            let version = WasmVersion(u32::from_le_bytes(wasm.slice()[4..8].try_into().unwrap()));

            let mut sections = vec![];
            loop {
                match Section::decode(&mut wasm) {
                    Ok(section) => {
                        sections.push(section);
                    }
                    Err(e) => {
                        if let DecoderError::InvalidCodeSize(_) = e {
                            return Ok(WasmModule { version, sections });
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
}

struct CodeSlice<'code> {
    code: &'code [u8],
    begin: usize,
    end: usize,
}

impl<'code> CodeSlice<'code> {
    pub fn try_new(code: &'code [u8], begin: usize, size: usize) -> Result<Self, DecoderError> {
        if code.len() <= begin || code.len() < begin + size {
            Err(DecoderError::InvalidCodeSize(code.len()))
        } else {
            Ok(Self {
                code,
                begin,
                end: begin + size,
            })
        }
    }

    pub fn slice(&self) -> &'code [u8] {
        self.code
    }

    pub fn try_next_with_size(&mut self, size: usize) -> Result<&mut Self, DecoderError> {
        if self.code.len() <= self.end || self.code.len() < self.end + size {
            Err(DecoderError::InvalidCodeSize(self.code.len()))
        } else {
            self.begin = self.end;
            self.end += size;

            Ok(self)
        }
    }
}
