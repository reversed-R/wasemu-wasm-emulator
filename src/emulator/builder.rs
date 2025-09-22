use crate::emulator::Emulator;

pub struct EmulatorBuilder {}

pub enum Linkable {
    Function,
    Memory,
}

impl EmulatorBuilder {
    pub fn load(wasm: &[u8]) -> Result<Self, ()> {
        todo!()
    }

    pub fn link(self, id: &str, linkable: Linkable) -> Result<Self, ()> {
        Ok(self)
    }

    pub fn build(self) -> Result<Emulator, ()> {
        todo!()
    }
}
