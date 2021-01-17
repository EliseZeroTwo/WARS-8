use std::sync::MutexGuard;

use crate::{palette::ColorPalette, MEM};

pub fn reset(mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    reset_palette(Some(mg), false);
    reset_palette(Some(mg), true);
}

pub fn reset_palette(mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>, draw: bool) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    for col in 0..16 {
        let offset;
        if draw {
            offset = (0x5f00 + col) as usize;
        } else {
            offset = (0x5f10 + col) as usize;
        }
        mg[offset] = col as u8
    }
}

pub fn get_draw_palette(
    mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>,
    col: ColorPalette,
) -> (ColorPalette, bool) {
    let mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mutex
        }
    };

    let idx = (0x5f00 + i32::from(col)) as usize;
    let val = mg[idx];
    let transparent = ((val >> 4) & 0b1) != 0;
    (ColorPalette::from((val & 0b1111) as i32), transparent)
}

pub fn set_draw_palette(
    mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>,
    col: ColorPalette,
    set_col: Option<ColorPalette>,
    set_transparent: Option<bool>,
) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    let idx = (0x5f00 + i32::from(col)) as usize;
    let mut val = mg[idx];

    if let Some(color) = set_col {
        val = (val & 0b1_0000) | i32::from(color) as u8;
    }

    if let Some(transparent) = set_transparent {
        val = (match transparent {
            false => 0,
            true => 1,
        } << 4)
            | (val & 0b1111);
    }

    mg[idx] = val;
}

pub fn get_screen_palette(
    mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>,
    col: ColorPalette,
) -> ColorPalette {
    let mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mutex
        }
    };

    let idx = (0x5f10 + i32::from(col)) as usize;
    let val = mg[idx];
    ColorPalette::from((val & 0b1111) as i32)
}

pub fn set_screen_palette(
    mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>,
    col: ColorPalette,
    set_col: ColorPalette,
) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    let idx = (0x5f10 + i32::from(col)) as usize;
    mg[idx] = i32::from(set_col) as u8;
}

pub fn get_pen_color(mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>) -> ColorPalette {
    let mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mutex
        }
    };

    ColorPalette::from((mg[0x5f25] & 0b1111) as i32)
}

pub fn set_pen_color(mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>, col: ColorPalette) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    let idx = 0x5f25;
    let old_val = mg[idx];

    mg[idx] = (old_val & 0b1111_0000) | i32::from(col) as u8;
}

pub fn get_print_cursor(mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>) -> (u8, u8) {
    let mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mutex
        }
    };

    let x = mg[0x5f26];
    let y = mg[0x5f27];

    (x, y)
}

pub fn set_print_cursor(
    mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>,
    x: Option<u8>,
    y: Option<u8>,
) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    if let Some(x) = x {
        mg[0x5f26] = x;
    }

    if let Some(y) = y {
        mg[0x5f27] = y;
    }
}

pub fn get_camera_offset(mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>) -> (i32, i32) {
    let mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mutex
        }
    };

    let x: u16 = ((mg[0x5f29] as u16) << 8) | mg[0x5f28] as u16;
    let y: u16 = ((mg[0x5f2b] as u16) << 8) | mg[0x5f2a] as u16;
    (x as i32, y as i32)
}

pub fn set_camera_offset(
    mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>,
    x: Option<i32>,
    y: Option<i32>,
) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    if let Some(x) = x {
        mg[0x5f28] = x as u8;
        mg[0x5f29] = (x >> 8) as u8;
    }

    if let Some(y) = y {
        mg[0x5f2a] = y as u8;
        mg[0x5f2b] = (y >> 8) as u8;
    }
}
