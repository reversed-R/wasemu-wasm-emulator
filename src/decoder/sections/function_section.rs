use crate::decoder::{DecoderError, RemainCode};

#[derive(Debug, Clone)]
pub struct FunctionSection {
    funcs: Vec<u32>,
}

impl FunctionSection {
    pub fn decode<'code>(
        wasm: RemainCode<'code>,
    ) -> Result<(Self, Option<RemainCode<'code>>), DecoderError> {
        let (count, mut wasm) = wasm.try_next_leb128()?; // count of section element; func
        let mut funcs = vec![];

        for _ in 0..count {
            let (func, remain_wasm) = wasm
                .ok_or(DecoderError::InvalidCodeSize)?
                .try_next_leb128()?;
            wasm = remain_wasm;
            funcs.push(func);
        }

        Ok((Self { funcs }, wasm))
    }
}
