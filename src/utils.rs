use crate::wasm_runtime::WasmCallerWrapper;
use std::ffi::CString;
pub struct FpsCounter(u32, u32);

impl FpsCounter {
    pub fn new(timer: u32) -> FpsCounter {
        FpsCounter((timer / 1000) % 10, 0)
    }

    pub fn tick(&mut self, timer: u32) {
        let sec = (timer / 1000) % 10;
        if sec != self.0 {
            println!("FPS: {}", self.1);
            self.1 = 0;
            self.0 = sec % 10;
        }
        self.1 += 1;
    }
}

pub fn read_cstr(caller_wrapper: &WasmCallerWrapper, addr: i32) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    let mut offset = 0;
    loop {
        let byte = caller_wrapper.peek(addr + offset);
        if byte == b'\x00' {
            break;
        }
        bytes.push(byte);
        offset += 1;
    }

    CString::new(bytes).unwrap().into_string().unwrap()
}
