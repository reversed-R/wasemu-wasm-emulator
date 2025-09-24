use crate::decoder::{DecoderError, WasmModule, instructions::Instr};

#[derive(Debug, Clone)]
struct ProgramCounter {
    func: usize,
    byte: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct JITDecoder<'code> {
    pc: ProgramCounter,
    module: WasmModule<'code>,
}

impl<'code> JITDecoder<'code> {
    pub(crate) fn new(module: WasmModule<'code>) -> Self {
        Self {
            pc: ProgramCounter { func: 0, byte: 0 }, // temporary default value set as 0, 0
            module,
        }
    }

    pub(crate) fn try_next(&mut self) -> Result<Instr, DecoderError> {
        Instr::decode(self)
    }

    pub(super) fn try_next_byte(&mut self) -> Result<u8, DecoderError> {
        if self.module.funcs[self.pc.func].code.code().len() <= self.pc.byte + 1 {
            Err(DecoderError::InvalidCodeSize)
        } else {
            self.pc.byte += 1;
            Ok(self.module.funcs[self.pc.func].code.code()[self.pc.byte])
        }
    }

    pub(super) fn try_next_le_u32(&mut self) -> Result<u32, DecoderError> {
        if self.module.funcs[self.pc.func].code.code().len() <= self.pc.byte + 4 {
            Err(DecoderError::InvalidCodeSize)
        } else {
            let last_byte = self.pc.byte;
            self.pc.byte += 4;
            Ok(u32::from_le_bytes(
                self.module.funcs[self.pc.func].code.code()[last_byte + 1..=self.pc.byte]
                    .try_into()
                    .unwrap(),
            ))
        }
    }

    pub(super) fn try_next_leb128(&mut self) -> Result<u32, DecoderError> {
        let code = self.module.funcs[self.pc.func].code.code();
        let mut acc: u32 = 0;
        let mut count: usize = 0;

        while count < code.len() {
            let b = code[self.pc.byte + count];
            let val: u32 = (b & 0b01111111) as u32;
            let shifted_val = val << (7 * count);
            acc += shifted_val;
            count += 1;
            if b < 0b10000000 {
                break;
            }
        }

        self.pc.byte += count;

        Ok(acc)
    }
}
