mod instructions;
pub mod jit;
mod sections;

use crate::decoder::sections::{
    SectionHeader, SectionId,
    code_section::{JITCodeBody, JITCodeSection},
    function_section::FunctionSection,
    type_section::{TypeBody, TypeSection},
};

#[derive(Debug, Clone, Copy)]
struct WasmVersion(u32);

#[derive(Debug, Clone)]
pub struct WasmModule<'code> {
    version: WasmVersion,
    funcs: Vec<WasmFunction<'code>>,
}

#[derive(Debug, Clone)]
pub struct WasmFunction<'code> {
    typ: TypeBody,
    code: JITCodeBody<'code>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderError {
    InvalidCodeSize,
    InvalidMagic,
    InvalidSectionCode,
    InvalidTypeCode,
    InvalidWasmType,
    InvalidSectionOrder,
    SectionDuplicated,
    FunctionTypeNotFound,
    NumberOfTypeSectionAndFunctionSectionMismatched,
    InvalidInstructionCode,
}

impl<'code> WasmModule<'code> {
    pub fn decode(wasm: &'code [u8]) -> Result<Self, DecoderError> {
        let wasm = RemainCode::new(wasm);
        let (magic, opt_wasm) = wasm.try_next_bytes(4)?;
        if magic != "\0asm".as_bytes() {
            return Err(DecoderError::InvalidMagic);
        }

        let wasm = opt_wasm.ok_or(DecoderError::InvalidCodeSize)?;

        let (version_u32, opt_wasm) = wasm.try_next_le_u32()?;
        let version = WasmVersion(version_u32);

        let mut wasm = if let Some(wasm) = opt_wasm {
            wasm
        } else {
            return Ok(WasmModule {
                version,
                funcs: vec![],
            });
        };
        let mut minsid = SectionId::Custom;

        let mut types: Option<Vec<TypeBody>> = None;
        let mut funcs: Option<Vec<u32>> = None;
        let mut codes: Option<Vec<JITCodeBody<'code>>> = None;
        loop {
            let (shdr, opt_wasm) = SectionHeader::decode(wasm)?;
            if let Some(remain_wasm) = opt_wasm {
                wasm = remain_wasm;
            } else {
                return Err(DecoderError::InvalidCodeSize);
            }

            if shdr.id.as_u8() != 0x00 {
                if shdr.id < minsid {
                    return Err(DecoderError::InvalidSectionOrder);
                } else if shdr.id == minsid {
                    return Err(DecoderError::SectionDuplicated);
                }
            }
            if shdr.id != SectionId::Custom {
                minsid = shdr.id;
            }

            let opt_wasm;
            match shdr.id {
                SectionId::Custom => {
                    todo!()
                }
                SectionId::Type => {
                    let section;
                    (section, opt_wasm) = TypeSection::decode(wasm)?;
                    types = Some(section.types);
                }
                SectionId::Import => {
                    todo!()
                }
                SectionId::Function => {
                    let section;
                    (section, opt_wasm) = FunctionSection::decode(wasm)?;
                    funcs = Some(section.funcs);
                }
                SectionId::Memory => {
                    todo!()
                }
                SectionId::Export => {
                    todo!()
                }
                SectionId::Code => {
                    let section;
                    (section, opt_wasm) = JITCodeSection::decode(wasm, shdr.size as usize)?;
                    codes = Some(section.funcs);
                }
                SectionId::Data => {
                    todo!()
                }
            }

            if let Some(remain_wasm) = opt_wasm {
                wasm = remain_wasm;
            } else if funcs.is_some() && codes.is_some() && types.is_some() {
                let funcs = funcs.unwrap();
                let codes = codes.unwrap();
                let types = types.unwrap();

                return Ok(WasmModule {
                    version,
                    funcs: funcs
                        .into_iter()
                        .zip(codes.into_iter())
                        .map(|(func, code)| {
                            if let Some(typ) = types.get(func as usize) {
                                Ok(WasmFunction {
                                    typ: typ.clone(),
                                    code,
                                })
                            } else {
                                Err(DecoderError::FunctionTypeNotFound)
                            }
                        })
                        .collect::<Result<Vec<WasmFunction>, DecoderError>>()?,
                });
            } else if funcs.is_none() && codes.is_none() {
                return Ok(WasmModule {
                    version,
                    funcs: vec![],
                });
            } else {
                return Err(DecoderError::NumberOfTypeSectionAndFunctionSectionMismatched);
            }
        }
    }
}

struct RemainCode<'code> {
    code: &'code [u8],
}

impl<'code> RemainCode<'code> {
    pub fn new(code: &'code [u8]) -> Self {
        Self { code }
    }

    pub fn try_next_bytes(self, size: usize) -> Result<(&'code [u8], Option<Self>), DecoderError> {
        if self.code.len() < size {
            Err(DecoderError::InvalidCodeSize)
        } else if self.code.len() == size {
            Ok((&self.code[..size], None))
        } else {
            Ok((
                &self.code[..size],
                Some(Self {
                    code: &self.code[size..],
                }),
            ))
        }
    }

    pub fn try_next_byte(self) -> Result<(u8, Option<Self>), DecoderError> {
        if self.code.is_empty() {
            Err(DecoderError::InvalidCodeSize)
        } else if self.code.len() == 1 {
            Ok((self.code[0], None))
        } else {
            Ok((
                self.code[0],
                Some(Self {
                    code: &self.code[1..],
                }),
            ))
        }
    }

    pub fn try_next_le_u32(self) -> Result<(u32, Option<Self>), DecoderError> {
        if self.code.len() < 4 {
            Err(DecoderError::InvalidCodeSize)
        } else {
            let u = u32::from_le_bytes(self.code[..4].try_into().unwrap());

            if self.code.len() == 4 {
                Ok((u, None))
            } else {
                Ok((
                    u,
                    Some(Self {
                        code: &self.code[4..],
                    }),
                ))
            }
        }
    }

    pub fn try_next_leb128(self) -> Result<(u32, Option<Self>), DecoderError> {
        let mut acc: u32 = 0;
        let mut count: usize = 0;

        while count < self.code.len() {
            let b = self.code[count];
            let val: u32 = (b & 0b01111111) as u32;
            let shifted_val = val << (7 * count);
            acc += shifted_val;
            count += 1;
            if b < 0b10000000 {
                break;
            }
        }

        if self.code.len() == count {
            Ok((acc, None))
        } else {
            Ok((
                acc,
                Some(Self {
                    code: &self.code[count..],
                }),
            ))
        }
    }
}
