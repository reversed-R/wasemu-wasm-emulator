use crate::decoder::{CodeSlice, DecoderError};

#[derive(Debug, Clone)]
pub struct TypeSection {}

impl TypeSection {
    pub fn decode(wasm: &mut CodeSlice) -> Result<Self, DecoderError> {
        todo!()
    }
}
