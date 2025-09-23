use crate::{decoder::WasmModule, emulator::Emulator};

pub struct EmulatorBuilder {
    wasm: WasmModule,
}

pub enum Linkable {
    Function,
    Memory,
}

impl EmulatorBuilder {
    pub fn load(wasm: &[u8]) -> Result<Self, ()> {
        let wasm = WasmModule::decode(wasm).expect("failed to decode");

        Ok(Self { wasm })
    }

    pub fn link(self, id: &str, linkable: Linkable) -> Result<Self, ()> {
        Ok(self)
    }

    pub fn build(self) -> Result<Emulator, ()> {
        Ok(Emulator { wasm: self.wasm })
    }
}
