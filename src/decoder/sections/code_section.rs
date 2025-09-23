use crate::decoder::{DecoderError, RemainCode};

#[derive(Debug, Clone)]
pub struct CodeSection {
    funcs: Vec<CodeBody>,
}

#[derive(Debug, Clone)]
pub struct CodeBody {}

#[derive(Debug, Clone)]
pub struct JITCodeSection<'code> {
    funcs: Vec<JITCodeBody<'code>>,
}

#[derive(Debug, Clone)]
pub struct JITCodeBody<'code> {
    code: &'code [u8],
}

impl<'code> JITCodeSection<'code> {
    pub fn decode(
        wasm: RemainCode<'code>,
        size: usize,
    ) -> Result<(Self, Option<RemainCode<'code>>), DecoderError> {
        let (count, mut wasm) = wasm.try_next_leb128()?; // count of section element; code
        let mut funcs = vec![];

        for _ in 0..count {
            let (func, remain_wasm) =
                JITCodeBody::decode(wasm.ok_or(DecoderError::InvalidCodeSize)?)?;
            wasm = remain_wasm;
            funcs.push(func);
        }

        Ok((Self { funcs }, wasm))
    }
}

impl<'code> JITCodeBody<'code> {
    pub fn decode(
        wasm: RemainCode<'code>,
    ) -> Result<(Self, Option<RemainCode<'code>>), DecoderError> {
        let (size, opt_wasm) = wasm.try_next_leb128()?;
        let (code, opt_wasm) = opt_wasm
            .ok_or(DecoderError::InvalidCodeSize)?
            .try_next_bytes(size as usize)?;

        Ok((Self { code }, opt_wasm))
    }
}
