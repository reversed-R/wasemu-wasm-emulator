use crate::decoder::{CodeSlice, DecoderError};

#[derive(Debug, Clone)]
pub struct FunctionSection {}

impl FunctionSection {
    pub fn decode(wasm: &mut CodeSlice) -> Result<Self, DecoderError> {
        todo!()
    }
}
