use crate::{
    runtime::wasm_runtime::WasmCallerWrapper, utils::read_cstr, ColorPallete, TerminalLocation,
    HEIGHT, PXBUF_MUTEX, WIDTH,
};
use font8x8::BASIC_UNICODE;
use wasmtime::*;

pub fn pset(x: i32, y: i32, color: i32) {
    let idx = usize::from(TerminalLocation(x, y));
    if idx >= (WIDTH * HEIGHT) as usize || x > 255 || y > 255 {
        panic!(format!("Invalid pset coordinates X={}, Y={}", x, y));
    }

    PXBUF_MUTEX.lock().unwrap()[idx] = ColorPallete::from(color);
}

pub fn pget(x: i32, y: i32) -> i32 {
    let idx = usize::from(TerminalLocation(x, y));
    if idx >= (WIDTH * HEIGHT) as usize || x < 0 || x > 255 || y < 0 || y > 255  {
        panic!(format!("Invalid pget coordinates X={}, Y={}", x, y));
    }

    i32::from(PXBUF_MUTEX.lock().unwrap()[idx])
}

pub fn rect(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut fb = PXBUF_MUTEX.lock().unwrap();

    if  x0 < 0 || x0 > 255 || y0 < 0 || y0 > 255 || x1 < 0 || x1 > 255 || y1 < 0 || y1 > 255  {
        panic!(format!("Invalid rect coordinates X0={}, Y0={}, X1={}, Y1={}", x0, y1, x1, y1));
    }

    for x in x0..=x1 {
        fb[usize::from(TerminalLocation(x, y0))] = ColorPallete::from(color);
        fb[usize::from(TerminalLocation(x, y1))] = ColorPallete::from(color);
    }

    for y in y0..=y1 {
        fb[usize::from(TerminalLocation(x0, y))] = ColorPallete::from(color);
        fb[usize::from(TerminalLocation(x1, y))] = ColorPallete::from(color);
    }
}

pub fn rectfill(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut fb = PXBUF_MUTEX.lock().unwrap();
    
    if  x0 < 0 || x0 > 255 || y0 < 0 || y0 > 255 || x1 < 0 || x1 > 255 || y1 < 0 || y1 > 255  {
        panic!(format!("Invalid rectfill coordinates X0={}, Y0={}, X1={}, Y1={}", x0, y1, x1, y1));
    }

    for x in x0..=x1 {
        for y in y0..=y1 {
            fb[usize::from(TerminalLocation(x, y))] = ColorPallete::from(color);
        }
    }
}

pub fn cls(color: i32) {
    rectfill(0, 0, WIDTH - 1, HEIGHT - 1, color);
}

pub fn putc(c: u8, x: i32, y: i32, col: i32) {
    if (c as usize) < BASIC_UNICODE.len() {
        let color = ColorPallete::from(col);
        let mut pxbuf_lock = PXBUF_MUTEX.lock().unwrap();
        let font = BASIC_UNICODE[c as usize];
        for row_offset in 0..8 {
            let row = font.byte_array()[row_offset];
            for bit in 0..8 {
                if (row >> (7 - bit)) & 1 == 1 {
                    for y_scale in 0..2 {
                        for x_scale in 0..2 {
                            let loc = TerminalLocation(
                                x + (bit * 2) as i32 + x_scale,
                                y + (row_offset * 2) as i32 + y_scale,
                            );
                            pxbuf_lock[usize::from(loc)] = color;
                        }
                    }
                }
            }
        }
    }
}

pub fn print(caller: Caller, string_addr: i32, x: i32, y: i32, col: i32) {
    let color = ColorPallete::from(col);
    let caller_wrapper = WasmCallerWrapper::new(caller);
    let mut pxbuf_lock = PXBUF_MUTEX.lock().unwrap();
    unsafe {
        let str = read_cstr(&caller_wrapper, string_addr);
        let mut offset = 0;
        for ch in str.as_bytes() {
            if offset >= 16 {
                break;
            }

            let char = *ch;
            if (char as usize) < BASIC_UNICODE.len() {
                let font = BASIC_UNICODE[char as usize];
                for row_offset in 0u8..8 {
                    let row = font.byte_array()[row_offset as usize];
                    for bit in 0u8..8 {
                        if (row >> bit) & 1 == 1 {
                            for y_scale in 0..2 {
                                for x_scale in 0..2 {
                                    let loc = TerminalLocation(
                                        x + (offset * 16) + (bit * 2) as i32 + x_scale,
                                        y + (row_offset * 2) as i32 + y_scale,
                                    );
                                    pxbuf_lock[usize::from(loc)] = color;
                                }
                            }
                        }
                    }
                }
            }
            offset += 1;
        }
    }
}

pub fn printh(caller: Caller, string_addr: i32) {
    println!(
        "{}",
        read_cstr(&WasmCallerWrapper::new(caller), string_addr)
    );
}
