use std::sync::MutexGuard;

use crate::{draw_state, font::FONT, get_map, set_map};
use crate::{
    get_pixel, get_sprite, get_sprite_flag, runtime::wasm_runtime::WasmCallerWrapper, set_pixel,
    set_sprite_flag, utils::read_cstr, ColorPalette, TerminalLocation, HEIGHT, MEM, WIDTH,
};

pub fn camera(x: i32, y: i32) {
    draw_state::set_camera_offset(None, Some(x), Some(y));
}

pub fn circ(x: i32, y: i32, r: i32, col: i32) {
    let col = ColorPalette::from(col);
    let mut x_off = 0;
    let mut y_off = r;
    let mut d = 1 - r;
    let mut mem = MEM.lock().unwrap();
    loop {
        for y_idx in &[-y_off, y_off] {
            for x_idx in &[-x_off, x_off] {
                let loc0 = TerminalLocation(x + x_idx, y + y_idx).apply_camera_offset(Some(&mem));
                let loc1 = TerminalLocation(x + y_idx, y + x_idx).apply_camera_offset(Some(&mem));
                set_pixel(Some(&mut mem), loc0, col);
                set_pixel(Some(&mut mem), loc1, col);
            }
        }

        if d < 0 {
            d += (2 * x_off) + 1;
        } else {
            y_off -= 1;
            d += (2 * x_off) - (2 * y_off) + 1;
        }
        x_off += 1;

        if x_off >= y_off {
            break;
        }
    }
}

pub fn circfill(x: i32, y: i32, r: i32, col: i32) {
    let col = ColorPalette::from(col);
    let mut x_off = 0;
    let mut y_off = r;
    let mut d = 1 - r;
    let mut mem = MEM.lock().unwrap();
    loop {
        for y_idx in -y_off..y_off {
            for x_idx in -x_off..x_off {
                let loc0 = TerminalLocation(x + x_idx, y + y_idx).apply_camera_offset(Some(&mem));
                let loc1 = TerminalLocation(x + y_idx, y + x_idx).apply_camera_offset(Some(&mem));
                set_pixel(Some(&mut mem), loc0, col);
                set_pixel(Some(&mut mem), loc1, col);
            }
        }

        if d < 0 {
            d += (2 * x_off) + 1;
        } else {
            y_off -= 1;
            d += (2 * x_off) - (2 * y_off) + 1;
        }
        x_off += 1;

        if x_off >= y_off {
            break;
        }
    }
}

pub fn clrpal() {
    let mut mem = MEM.lock().unwrap();
    draw_state::reset_palette(Some(&mut mem), true);
    draw_state::reset_palette(Some(&mut mem), false);
}

pub fn pal(c0: i32, c1: i32, p: i32) {
    let col0 = ColorPalette::from(c0);
    let col1 = ColorPalette::from(c1);
    if p == 0 {
        draw_state::set_draw_palette(None, col0, Some(col1), None);
    } else if p == 1 {
        draw_state::set_screen_palette(None, col0, col1);
    }
}

pub fn palt(col: i32, transparent: i32) {
    let transparent = transparent != 0;
    draw_state::set_draw_palette(None, ColorPalette::from(col), None, Some(transparent));
}

pub fn pset(x: i32, y: i32, color: i32) {
    let idx = TerminalLocation(x, y);
    let mut mem = MEM.lock().unwrap();
    let col = ColorPalette::from(color).apply_palette_mod(Some(&mem), false);
    set_pixel(Some(&mut mem), idx, col);
}

pub fn pget(x: i32, y: i32) -> i32 {
    let idx = TerminalLocation(x, y);
    i32::from(get_pixel(None, idx))
}

pub fn rect(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut mem = MEM.lock().unwrap();
    let color = ColorPalette::from(color).apply_palette_mod(Some(&mem), false);
    for x in x0..=x1 {
        let y0_loc = TerminalLocation(x, y0).apply_camera_offset(Some(&mem));
        let y1_loc = TerminalLocation(x, y1).apply_camera_offset(Some(&mem));

        set_pixel(Some(&mut mem), y0_loc, color);
        set_pixel(Some(&mut mem), y1_loc, color);
    }

    for y in y0..=y1 {
        let x0_loc = TerminalLocation(x0, y).apply_camera_offset(Some(&mem));
        let x1_loc = TerminalLocation(x1, y).apply_camera_offset(Some(&mem));
        set_pixel(Some(&mut mem), x0_loc, color);
        set_pixel(Some(&mut mem), x1_loc, color);
    }
}

pub fn rectfill(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut mem = MEM.lock().unwrap();
    let color = ColorPalette::from(color).apply_palette_mod(Some(&mem), false);
    for x in x0..=x1 {
        for y in y0..=y1 {
            let loc = TerminalLocation(x, y).apply_camera_offset(Some(&mem));
            set_pixel(Some(&mut mem), loc, color);
        }
    }
}

pub fn cls(color: i32) {
    rectfill(0, 0, WIDTH - 1, HEIGHT - 1, color);
}

pub fn print(string: String, x: i32, y: i32, col: i32) {
    let string = string.to_ascii_uppercase();
    let mut mem = MEM.lock().unwrap();
    let color = ColorPalette::from(col).apply_palette_mod(Some(&mem), false);
    unsafe {
        let mut offset = 0;
        for ch in string.chars().collect::<Vec<char>>() {
            if let Some(font) = FONT.get(&ch) {
                for row_offset in 0..6 {
                    let row = font[row_offset];
                    for col_idx in 0..4 {
                        if row[col_idx] {
                            let loc = TerminalLocation(
                                x + (offset * 4) + col_idx as i32,
                                y + row_offset as i32,
                            )
                            .apply_camera_offset(Some(&mem));
                            set_pixel(Some(&mut mem), loc, color);
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

pub fn spr(
    mem: Option<&mut MutexGuard<[u8; 0x8000]>>,
    idx: i32,
    x: i32,
    y: i32,
    w: f32,
    h: f32,
    flip_x: i32,
    flip_y: i32,
) {
    let width_px = (8.0 * w).floor() as i32;
    let height_px = (8.0 * h).floor() as i32;

    let flip_x = flip_x != 0;
    let flip_y = flip_y != 0;

    let mut mutex;
    let mem = match mem {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    for height_idx in 0..=(height_px / 8) {
        for width_idx in 0..=(width_px / 8) {
            let sprite = get_sprite(Some(&mem), (height_idx * 0x10) + idx + width_idx);
            {
                let row_px: i32 = (width_px - (8 * width_idx)).min(8).max(-8);
                let height_px: i32 = (height_px - (8 * height_idx)).min(8).max(-8);

                for row_idx in 0..height_px {
                    for col_idx in 0..row_px {
                        let loc = TerminalLocation(
                            x + (width_idx * 8) + col_idx,
                            y + (height_idx * 8) + row_idx,
                        )
                        .apply_camera_offset(Some(mem));
                        let col = sprite[row_idx as usize][col_idx as usize]
                            .apply_palette_mod(Some(mem), false);
                        if col != ColorPalette::Black {
                            set_pixel(Some(mem), loc, col);
                        }
                    }
                }
            }
        }
    }
}

pub fn sspr(
    sx: i32,
    sy: i32,
    sw: i32,
    sh: i32,
    dx: i32,
    dy: i32,
    dw: i32,
    dh: i32,
    flip_x: i32,
    flip_y: i32,
) {
    let mut mem = MEM.lock().unwrap();
    let width_px = dw * 8;
    let height_px = dh * 8;

    let flip_x = flip_x != 0;
    let flip_y = flip_y != 0;

    for row_offset in 0..sh {
        for col_offset in 0..sw {
            for y_str_offset in 0..dh {
                for x_str_offset in 0..dw {
                    let loc = TerminalLocation(
                        dx + (col_offset * dw) + x_str_offset,
                        dy + (row_offset * dh) + y_str_offset,
                    )
                    .apply_camera_offset(Some(&mem));
                    let offset = ((sx + col_offset) / 2) + ((sy + row_offset) * 64);
                    let col = ColorPalette::from(mem[offset as usize] as i32)
                        .apply_palette_mod(Some(&mem), false);
                    if col != ColorPalette::Black {
                        set_pixel(Some(&mut mem), loc, col);
                    }
                }
            }
        }
    }
}

pub fn map(cel_x: i32, cel_y: i32, scr_x: i32, scr_y: i32, cel_w: i32, cel_h: i32, layer: i32) {
    let mut mem = MEM.lock().unwrap();
    let layer = layer as u8;
    for y in 0..cel_h {
        for x in 0..cel_w {
            let loc = TerminalLocation((scr_y + (y * 8)) as i32, (scr_x + (x * 8)) as i32)
                .apply_camera_offset(Some(&mem));
            if loc.is_valid() {
                let val = get_map(Some(&mut mem), cel_x + x, cel_y + y) as i32;
                if get_sprite_flag(Some(&mem), val, None) & layer == layer {
                    spr(
                        Some(&mut mem),
                        val,
                        scr_x + (x * 8),
                        scr_y + (y * 8),
                        1.0,
                        1.0,
                        0,
                        0,
                    );
                }
            }
        }
    }
}

pub fn mget(x: i32, y: i32) -> u8 {
    get_map(None, x, y)
}

pub fn mset(x: i32, y: i32, val: u8) {
    set_map(None, x, y, val);
}
