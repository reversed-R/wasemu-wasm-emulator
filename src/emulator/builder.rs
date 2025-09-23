use crate::{decoder::WasmModule, emulator::Emulator};

pub struct EmulatorBuilder<'code> {
    wasm: WasmModule<'code>,
}

pub enum Linkable {
    Function,
    Memory,
}

impl<'code> EmulatorBuilder<'code> {
    pub fn load(wasm: &'code [u8]) -> Result<Self, ()> {
        let wasm = WasmModule::decode(wasm).expect("failed to decode");

        Ok(Self { wasm })
    }

    pub fn link(self, id: &str, linkable: Linkable) -> Result<Self, ()> {
        Ok(self)
    }

    pub fn build(self) -> Result<Emulator<'code>, ()> {
        Ok(Emulator { wasm: self.wasm })
    }
}
