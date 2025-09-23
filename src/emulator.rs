use crate::decoder::WasmModule;

pub mod builder;

#[derive(Debug, Clone)]
pub struct Emulator {
    wasm: WasmModule,
}

pub struct RestartableEmulator(Emulator);

impl Emulator {
    pub fn start(&mut self) -> Result<(), ()> {
        Ok(())
    }

    pub fn start_until_limit(self, limit: usize) -> Result<RestartableEmulator, ()> {
        Ok(RestartableEmulator(self))
    }

    pub fn exec_func(&mut self, id: &str) -> Result<(), ()> {
        Ok(())
    }

    pub fn exec_func_until_limit(self, id: &str, limit: usize) -> Result<RestartableEmulator, ()> {
        Ok(RestartableEmulator(self))
    }

    fn restart(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn restart_until_limit(self, limit: usize) -> Result<RestartableEmulator, ()> {
        Ok(RestartableEmulator(self))
    }
}

impl RestartableEmulator {
    pub fn restart(&mut self) -> Result<(), ()> {
        self.0.restart()
    }

    pub fn restart_until_limit(self, limit: usize) -> Result<RestartableEmulator, ()> {
        self.0.restart_until_limit(limit)
    }
}
