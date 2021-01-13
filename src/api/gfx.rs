use crate::{
    runtime::wasm_runtime::WasmCallerWrapper, utils::read_cstr, ColorPallete, TerminalLocation,
    HEIGHT, PXBUF_MUTEX, WIDTH,
};
use crate::font::FONT;

pub fn pset(x: i32, y: i32, color: i32) {
    let idx = usize::from(TerminalLocation(x, y));
    if idx >= (WIDTH * HEIGHT) as usize || x > 255 || y > 255 {
        panic!(format!("Invalid pset coordinates X={}, Y={}", x, y));
    }

    PXBUF_MUTEX.lock().unwrap()[idx] = ColorPallete::from(color);
}

pub fn pget(x: i32, y: i32) -> i32 {
    let idx = usize::from(TerminalLocation(x, y));
    if idx >= (WIDTH * HEIGHT) as usize || x < 0 || x >= WIDTH || y < 0 || y >= HEIGHT  {
        panic!(format!("Invalid pget coordinates X={}, Y={}", x, y));
    }

    i32::from(PXBUF_MUTEX.lock().unwrap()[idx])
}

pub fn rect(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut fb = PXBUF_MUTEX.lock().unwrap();

    if  x0 < 0 || x0 >= WIDTH || y0 < 0 || y0 >= HEIGHT || x1 < 0 || x1 >= WIDTH || y1 < 0 || y1 >= HEIGHT  {
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
    
    if  x0 < 0 || x0 >= WIDTH || y0 < 0 || y0 >= HEIGHT || x1 < 0 || x1 >= WIDTH || y1 < 0 || y1 >= HEIGHT  {
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

pub fn print(string: String, x: i32, y: i32, col: i32) {
    let color = ColorPallete::from(col);
    let string = string.to_ascii_uppercase();
    let mut pxbuf_lock = PXBUF_MUTEX.lock().unwrap();
    unsafe {
        let mut offset = 0;
        for ch in string.chars().collect::<Vec<char>>() {
            if x + (offset * 4) + 3 > 127 || x + (offset * 4) + 3 < 0 || y - 6 < 0 || y > 127 {
                return;
            }

            if let Some(font) = FONT.get(&ch) {
                for row_offset in 0..6 {
                    let row = font[5 - row_offset];
                    for col_idx in 0..4 {
                        if row[col_idx] {
                                let loc = TerminalLocation(
                                    x + (offset * 4) + col_idx as i32,
                                    y - row_offset as i32,
                                );
                                pxbuf_lock[usize::from(loc)] = color;
                        }
                    }
                }
            }
            offset += 1;
        }
    }
}

pub fn printh(str: String) {
    println!(
        "{}",
        str
    );
}
