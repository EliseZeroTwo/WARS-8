use crate::wasm_runtime::WasmCallerWrapper;
use std::ffi::CString;

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
