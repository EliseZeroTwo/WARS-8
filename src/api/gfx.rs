use crate::font::FONT;
use crate::{
    runtime::wasm_runtime::WasmCallerWrapper, utils::read_cstr, ColorPallete, TerminalLocation,
    HEIGHT, PXBUF_MUTEX, WIDTH,
};

pub fn pset(x: i32, y: i32, color: i32) {
    let idx = TerminalLocation(x, y);
    if idx.is_valid() {
        PXBUF_MUTEX.lock().unwrap()[usize::from(idx)] = ColorPallete::from(color);
    }
}

pub fn pget(x: i32, y: i32) -> i32 {
    let idx = TerminalLocation(x, y);
    if idx.is_valid() {
        i32::from(PXBUF_MUTEX.lock().unwrap()[usize::from(idx)])
    } else {
        0
    }
}

pub fn rect(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut fb = PXBUF_MUTEX.lock().unwrap();

    for x in x0..=x1 {
        let y0_loc = TerminalLocation(x, y0);
        let y1_loc = TerminalLocation(x, y1);

        if y0_loc.is_valid() {
            fb[usize::from(y0_loc)] = ColorPallete::from(color);
        }

        if y1_loc.is_valid() {
            fb[usize::from(y1_loc)] = ColorPallete::from(color);
        }
    }

    for y in y0..=y1 {
        let x0_loc = TerminalLocation(x0, y);
        let x1_loc = TerminalLocation(x1, y);
        if x0_loc.is_valid() {
            fb[usize::from(x0_loc)] = ColorPallete::from(color);
        }

        if x1_loc.is_valid() {
            fb[usize::from(x1_loc)] = ColorPallete::from(color);
        }
    }
}

pub fn rectfill(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut fb = PXBUF_MUTEX.lock().unwrap();

    for x in x0..=x1 {
        for y in y0..=y1 {
            let loc = TerminalLocation(x, y);
            if loc.is_valid() {
                fb[usize::from(loc)] = ColorPallete::from(color);
            }
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
            if let Some(font) = FONT.get(&ch) {
                for row_offset in 0..6 {
                    let row = font[5 - row_offset];
                    for col_idx in 0..4 {
                        if row[col_idx] {
                            let loc = TerminalLocation(
                                x + (offset * 4) + col_idx as i32,
                                y - row_offset as i32,
                            );
                            if loc.is_valid() {
                                pxbuf_lock[usize::from(loc)] = color;
                            }
                        }
                    }
                }
            }
            offset += 1;
        }
    }
}

pub fn printh(str: String) {
    println!("{}", str);
}

pub fn spr(idx: i32, x: i32, y: i32, w: f32, h: f32, flip_x: i32, flip_y: i32) {
    let mutex = crate::CART.lock().unwrap();
    let width_px = (8.0 * w).floor() as i32;
    let height_px = (8.0 * h).floor() as i32;

    let flip_x = flip_x != 0;
    let flip_y = flip_y != 0;

    let mut pxbuf_lock = PXBUF_MUTEX.lock().unwrap();

    for height_idx in 0..=(height_px / 8) {
        for width_idx in 0..=(width_px / 8) {
            if let Some(sprite) = mutex
                .as_deref()
                .unwrap()
                .get_sprite((height_idx * 0x10) + idx + width_idx)
            {
                let row_px: i32 = (width_px - (8 * width_idx)).min(8).max(-8);
                let height_px: i32 = (height_px - (8 * height_idx)).min(8).max(-8);

                for row_idx in 0..height_px {
                    for col_idx in 0..row_px {
                        let loc = TerminalLocation(
                            x + (width_idx * 8) + col_idx,
                            y + (height_idx * 8) + row_idx,
                        );
                        if loc.is_valid() {
                            pxbuf_lock[usize::from(loc)] =
                                sprite[row_idx as usize][col_idx as usize];
                        }
                    }
                }
            }
        }
    }
}
