use crate::decoder::{DecoderError, RemainCode};

#[derive(Debug, Clone)]
pub struct TypeSection {
    types: Vec<TypeBody>,
}

#[derive(Debug, Clone)]
pub enum TypeBody {
    Function(WasmFunctionType),
}

#[derive(Debug, Clone)]
pub struct WasmFunctionType {
    params: Vec<WasmNumberType>,
    results: Vec<WasmNumberType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmNumberType {
    F64,
    F32,
    I64,
    I32,
}

impl TypeSection {
    pub fn decode<'code>(
        wasm: RemainCode<'code>,
    ) -> Result<(Self, Option<RemainCode<'code>>), DecoderError> {
        let (count, mut wasm) = wasm.try_next_leb128()?; // count of section element; type
        let mut types = vec![];

        for _ in 0..count {
            let (typ, wasm_remain) = TypeBody::decode(wasm.ok_or(DecoderError::InvalidCodeSize)?)?;
            wasm = wasm_remain;
            types.push(typ);
        }

        Ok((Self { types }, wasm))
    }
}

impl TypeBody {
    pub fn decode<'code>(
        wasm: RemainCode<'code>,
    ) -> Result<(Self, Option<RemainCode<'code>>), DecoderError> {
        let (type_code, wasm) = wasm.try_next_byte()?;

        match type_code {
            // func
            0x60 => {
                let (params_count, mut wasm) = wasm
                    .ok_or(DecoderError::InvalidCodeSize)?
                    .try_next_leb128()?;
                let mut params = vec![];
                for _ in 0..params_count {
                    let (param_code, remain_wasm) =
                        wasm.ok_or(DecoderError::InvalidCodeSize)?.try_next_byte()?;
                    wasm = remain_wasm;

                    params.push(WasmNumberType::try_from(param_code)?);
                }

                let (results_count, mut wasm) = wasm
                    .ok_or(DecoderError::InvalidCodeSize)?
                    .try_next_leb128()?;
                let mut results = vec![];
                for _ in 0..results_count {
                    let (result_code, remain_wasm) =
                        wasm.ok_or(DecoderError::InvalidCodeSize)?.try_next_byte()?;
                    wasm = remain_wasm;
                    results.push(WasmNumberType::try_from(result_code)?);
                }

                Ok((Self::Function(WasmFunctionType { params, results }), wasm))
            }
            _ => Err(DecoderError::InvalidTypeCode),
        }
    }
}

impl TryFrom<u8> for WasmNumberType {
    type Error = DecoderError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7c => Ok(Self::F64),
            0x7d => Ok(Self::F32),
            0x7e => Ok(Self::I64),
            0x7f => Ok(Self::I32),
            _ => Err(DecoderError::InvalidWasmType),
        }
    }
}
