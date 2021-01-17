#![feature(slice_fill)]

extern crate directories;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate rand_pcg;
extern crate regex;
extern crate sdl2;
extern crate serde_json;
extern crate wasmtime;

mod cart;
mod config;
mod draw_state;
mod font;
mod palette;
mod runtime;
mod utils;

// Api
mod api;

use crate::cart::Cart;
use crate::config::Config;
use crate::palette::ColorPalette;
use crate::runtime::*;
use crate::utils::*;
use rand_pcg::Pcg64Mcg;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::{
    event::{Event, WindowEvent},
    rect::Rect,
};
use std::convert::From;
use std::sync::Mutex;
use std::{collections::HashSet, sync::MutexGuard};

const WINDOW_WIDTH: i32 = 512;
const WINDOW_HEIGHT: i32 = 512;
const WIDTH: i32 = 128;
const HEIGHT: i32 = 128;
const TARGET_FPS: f32 = 30.0;
const FRAME_LEN_MS: u32 = ((1.0 / TARGET_FPS) * 1000.0) as u32;

#[derive(Copy, Clone, Debug)]
pub struct TerminalLocation(pub i32, pub i32);

impl From<TerminalLocation> for usize {
    fn from(loc: TerminalLocation) -> Self {
        ((loc.1 * WIDTH) + loc.0) as usize
    }
}

impl TerminalLocation {
    fn is_valid(&self) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < WIDTH && self.1 < HEIGHT
    }

    fn apply_camera_offset(
        &self,
        mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>,
    ) -> TerminalLocation {
        let off = crate::draw_state::get_camera_offset(mutex_guard);
        TerminalLocation(self.0 - off.0, self.1 - off.1)
    }
}

lazy_static! {
    static ref KEYSTATE_FRAME: Mutex<HashSet<Scancode>> = Mutex::new(HashSet::new());
    static ref KEYSTATE_FRAME_FIFO: Mutex<Vec<Scancode>> = Mutex::new(Vec::new());
    static ref KEYSTATE_HELD: Mutex<HashSet<Scancode>> = Mutex::new(HashSet::new());
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::get_config_or_create());
    static ref RAND_SRC: Mutex<Pcg64Mcg> = Mutex::new(Pcg64Mcg::new(0xcafef00dbeefd34d));
    static ref CART: Mutex<Option<Box<dyn Cart>>> = Mutex::new(None);
    static ref CART_TO_LOAD: Mutex<bool> = Mutex::new(false);
    static ref MEM: Mutex<[u8; 0x8000]> = Mutex::new([0; 0x8000]);
}

pub fn set_pixel(
    mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>,
    loc: TerminalLocation,
    color: ColorPalette,
) {
    if loc.is_valid() {
        let mut mutex;
        let mg = match mutex_guard {
            Some(mg) => mg,
            None => {
                mutex = MEM.lock().unwrap();
                &mut mutex
            }
        };

        let offset = (0x6000 + loc.0 / 2 + (loc.1 * WIDTH / 2)) as usize;
        if loc.0 % 2 == 1 {
            let lower = mg[offset] & 0b1111;
            mg[offset] = (i32::from(color) as u8) << 4 | lower;
        } else {
            let higher = mg[offset] & 0b1111_0000;
            mg[offset] = (i32::from(color) as u8) | higher;
        }
    }
}

pub fn get_pixel(
    mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>,
    loc: TerminalLocation,
) -> ColorPalette {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    let offset = (0x6000 + loc.0 / 2 + (loc.1 * WIDTH / 2)) as usize;
    if loc.0 % 2 == 1 {
        ColorPalette::from(((mg[offset] >> 4) & 0b1111) as i32)
    } else {
        ColorPalette::from((mg[offset] & 0b1111) as i32)
    }
}

pub fn set_map(mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>, x: i32, y: i32, val: u8) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    if y < 32 {
        mg[(0x2000 + x + (y * 128)) as usize] = val;
    } else if y < 64 {
        mg[(0x1000 + x + (y * 128)) as usize] = val;
    }
}

pub fn get_map(mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>, x: i32, y: i32) -> u8 {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    if y < 32 {
        mg[(0x2000 + x + (y * 128)) as usize]
    } else if y < 64 {
        mg[(0x1000 + x + (y * 128)) as usize]
    } else {
        0
    }
}

pub fn set_sprite(
    mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>,
    idx: i32,
    data: [[ColorPalette; 8]; 8],
) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    let offset = (4 * (idx % 16)) + ((8 * 64) * (idx / 16));
    for y in 0..8 {
        for x in 0..4 {
            let offset = (offset + x + (y * 64)) as usize;

            let col0 = i32::from(data[y as usize][(x * 2) as usize]) as u8;
            let col1 = i32::from(data[y as usize][((x * 2) + 1) as usize]) as u8;
            mg[offset as usize] = col0 | (col1 << 4);
        }
    }
}

pub fn get_sprite(
    mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>,
    idx: i32,
) -> [[ColorPalette; 8]; 8] {
    let mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mutex
        }
    };

    let mut sprite = [[ColorPalette::Black; 8]; 8];
    let offset = (4 * (idx % 16)) + ((8 * 64) * (idx / 16));
    for y in 0..8 {
        for x in 0..4 {
            let offset = offset + x + (y * 64);

            let col0 = mg[offset as usize] & 0b1111;
            let col1 = (mg[offset as usize] >> 4) & 0b1111;
            sprite[y as usize][(x * 2) as usize] = ColorPalette::from(col0 as i32);
            sprite[y as usize][(x * 2) as usize] = ColorPalette::from(col1 as i32);
        }
    }

    sprite
}

pub fn set_sprite_flag(
    mutex_guard: Option<&mut MutexGuard<[u8; 0x8000]>>,
    sprite_idx: i32,
    flag_idx: Option<u8>,
    val: u8,
) {
    let mut mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mut mutex
        }
    };

    if sprite_idx >= 0 && sprite_idx <= 0xFF {
        let addr = 0x3000 + sprite_idx as usize;
        if let Some(idx) = flag_idx {
            let idx = idx.max(7);
            let set = (val & 0b1) << idx;
            if set != 0 {
                mg[addr] |= set;
            } else {
                mg[addr] &= !(1 << idx);
            }
        } else {
            mg[addr] = val;
        }
    }
}

pub fn get_sprite_flag(
    mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>,
    sprite_idx: i32,
    flag_idx: Option<u8>,
) -> u8 {
    let mutex;
    let mg = match mutex_guard {
        Some(mg) => mg,
        None => {
            mutex = MEM.lock().unwrap();
            &mutex
        }
    };

    if sprite_idx >= 0 && sprite_idx <= 0xFF {
        let val = mg[0x3000 + sprite_idx as usize];
        if let Some(idx) = flag_idx {
            val >> idx.max(7) & 0b1
        } else {
            val
        }
    } else {
        0
    }
}

fn main() {
    let mut boot_cart_path;
    if std::env::args().len() == 1 {
        let path_obj =
            std::path::Path::new(&Config::get_config_dir_or_create().unwrap()).join("boot.wasm");
        boot_cart_path = path_obj.to_str().unwrap().to_string();
        if !path_obj.is_file() {
            println!(
                "Usage: `{} <binary>`, or place a binary at `{}`",
                std::env::args().next().unwrap(),
                boot_cart_path
            );
            std::process::exit(1);
        }
    } else {
        boot_cart_path = "".to_owned();
        for x in std::env::args() {
            boot_cart_path = x;
        }
    }

    // Setup SDL
    let sdl_ctx = sdl2::init().unwrap();
    let window = sdl_ctx
        .video()
        .unwrap()
        .window("WARS-8", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut out_win_rect = Rect::new(0, 0, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBX8888, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let mut cart_pre_mutex = CART.lock().unwrap();
    *cart_pre_mutex = Some(Cart::load(&boot_cart_path));
    let mut runtime = cart_pre_mutex.as_deref().unwrap().create_runtime();
    drop(cart_pre_mutex);

    draw_state::reset(None);
    runtime.init();

    let mut target_ms = sdl_ctx.timer().unwrap().ticks() + FRAME_LEN_MS;
    let mut fps_counter = FpsCounter::new(sdl_ctx.timer().unwrap().ticks());
    let mut paused = false;
    'sdlloop: loop {
        let mut cart_mutex = CART.lock().unwrap();
        let mut cart_to_load_mutex = CART_TO_LOAD.lock().unwrap();
        if cart_mutex.is_none() || *cart_to_load_mutex == true {
            let mut mem = MEM.lock().unwrap();
            mem.fill(0);

            draw_state::reset(Some(&mut mem));

            *cart_to_load_mutex = false;
            if cart_mutex.is_none() {
                println!("Resetting to boot cartridge");
                *cart_mutex = Some(Cart::load(&boot_cart_path));
            }

            runtime = cart_mutex.as_deref().unwrap().create_runtime();
            runtime.init();
        }

        drop(cart_mutex);
        drop(cart_to_load_mutex);

        runtime.update();
        runtime.draw();

        let config = CONFIG.lock().unwrap();
        let mut keystate_frame = KEYSTATE_FRAME.lock().unwrap();
        let mut keystate_frame_fifo = KEYSTATE_FRAME_FIFO.lock().unwrap();
        let mut keystate_held = KEYSTATE_HELD.lock().unwrap();

        keystate_frame.clear();
        keystate_frame_fifo.clear();

        for event in sdl_ctx.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'sdlloop;
                }
                Event::KeyDown {
                    scancode: Some(kc), ..
                } if kc as i32 == config.keys.quit => {
                    let mut cart_mutex = CART.lock().unwrap();
                    let mut cart_to_load_mutex = CART_TO_LOAD.lock().unwrap();
                    *cart_to_load_mutex = true;
                    *cart_mutex = Some(Cart::load(&boot_cart_path));
                }
                Event::KeyDown {
                    scancode: Some(kc), ..
                } if kc as i32 == config.keys.pause => {
                    paused = !paused;
                }

                Event::KeyDown {
                    scancode: Some(kc), ..
                } => {
                    keystate_frame.insert(kc);
                    if !keystate_frame_fifo.contains(&kc) {
                        keystate_frame_fifo.push(kc);
                    }

                    if !keystate_held.contains(&kc) {
                        keystate_held.insert(kc);
                    }
                }

                Event::KeyUp {
                    scancode: Some(kc), ..
                } => {
                    keystate_held.remove(&kc);
                }

                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(new_x, new_y) = win_event {
                        let screen_size_scaled = new_x.min(new_y);
                        let x = (((new_x - screen_size_scaled) >> 1) << 1) / 2;
                        let y = (((new_y - screen_size_scaled) >> 1) << 1) / 2;
                        out_win_rect =
                            Rect::new(x, y, screen_size_scaled as u32, screen_size_scaled as u32);
                    }
                }

                _ => {}
            }
        }

        drop(config);
        drop(keystate_held);
        drop(keystate_frame);
        drop(keystate_frame_fifo);

        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                let memory = MEM.lock().unwrap();
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let loc = TerminalLocation(x, y);
                        let raw_idx = usize::from(loc) * 4;
                        let color = Color::from(get_pixel(Some(&memory), loc));
                        buffer[raw_idx + 1] = color.b;
                        buffer[raw_idx + 2] = color.g;
                        buffer[raw_idx + 3] = color.r;
                    }
                }
            })
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, Some(out_win_rect)).unwrap();
        canvas.present();

        let frame_difference = target_ms as i32 - sdl_ctx.timer().unwrap().ticks() as i32;
        target_ms += FRAME_LEN_MS;
        if frame_difference > 0 {
            sdl_ctx.timer().unwrap().delay(frame_difference as u32);
        }
        fps_counter.tick(sdl_ctx.timer().unwrap().ticks());
    }

    std::fs::write("./lastmem.bin", *MEM.lock().unwrap()).unwrap();
}
