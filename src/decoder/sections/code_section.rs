use crate::decoder::{CodeSlice, DecoderError};

#[derive(Debug, Clone)]
pub struct CodeSection {}

impl CodeSection {
    pub fn decode(wasm: &mut CodeSlice) -> Result<Self, DecoderError> {
        todo!()
    }
}
