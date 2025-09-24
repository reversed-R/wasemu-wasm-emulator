use crate::decoder::jit::JITDecoder;

pub mod builder;

#[derive(Debug, Clone)]
pub struct Emulator<'code> {
    code: JITDecoder<'code>,
}

pub struct RestartableEmulator<'code>(Emulator<'code>);

impl<'code> Emulator<'code> {
    pub fn start(&mut self) -> Result<(), ()> {
        loop {
            match self.code.try_next() {
                Ok(instr) => {
                    eprintln!("instr: {instr:?}");
                }
                Err(e) => {
                    eprintln!("error: {e:?}");
                    return Ok(());
                }
            }
        }
    }

    pub fn start_until_limit(self, limit: usize) -> Result<RestartableEmulator<'code>, ()> {
        Ok(RestartableEmulator(self))
    }

    pub fn exec_func(&mut self, id: &str) -> Result<(), ()> {
        Ok(())
    }

    pub fn exec_func_until_limit(
        self,
        id: &'static str,
        limit: usize,
    ) -> Result<RestartableEmulator<'code>, ()> {
        Ok(RestartableEmulator(self))
    }

    fn restart(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn restart_until_limit(self, limit: usize) -> Result<RestartableEmulator<'code>, ()> {
        Ok(RestartableEmulator(self))
    }
}

impl<'code> RestartableEmulator<'code> {
    pub fn restart(&mut self) -> Result<(), ()> {
        self.0.restart()
    }

    pub fn restart_until_limit(self, limit: usize) -> Result<RestartableEmulator<'code>, ()> {
        self.0.restart_until_limit(limit)
    }
}
