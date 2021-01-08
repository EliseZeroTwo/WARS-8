extern crate directories;
extern crate font8x8;
#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate serde_json;
extern crate wasmtime;

macro_rules! func_wrap {
    ($wasm_runtime:expr, $func:expr) => {
        Func::wrap(&$wasm_runtime.store.as_ref().unwrap(), $func).into()
    };
}

mod config;
mod palette;
mod runtime;
mod utils;
mod wasm_runtime;

use crate::config::Config;
use crate::palette::ColorPallete;
use crate::runtime::*;
use crate::utils::*;
use crate::wasm_runtime::{WasmCallerWrapper, WasmRuntime};
use font8x8::unicode::BASIC_UNICODE;
use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::convert::From;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;
use wasmtime::*;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;
const TARGET_FPS: f32 = 30.0;
const FRAME_LEN_MS: u32 = ((1.0 / TARGET_FPS) * 1000.0) as u32;

#[derive(Copy, Clone, Debug)]
pub struct TerminalLocation(pub i32, pub i32);

impl From<TerminalLocation> for usize {
    fn from(loc: TerminalLocation) -> Self {
        ((loc.1 * WIDTH) + loc.0) as usize
    }
}

#[derive(Copy, Clone, Debug)]
pub struct KeyState {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    o: bool,
    x: bool,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            left: false,
            right: false,
            up: false,
            down: false,
            o: false,
            x: false,
        }
    }

    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.up = false;
        self.down = false;
        self.o = false;
        self.x = false;
    }
}

lazy_static! {
    static ref PXBUF_MUTEX: Mutex<[ColorPallete; (WIDTH * HEIGHT) as usize]> =
        Mutex::new([ColorPallete::Black; (WIDTH * HEIGHT) as usize]);
    static ref KEYSTATE_FRAME: Mutex<HashSet<Scancode>> = Mutex::new(HashSet::new());
    static ref KEYSTATE_FRAME_FIFO: Mutex<Vec<Scancode>> = Mutex::new(Vec::new());
    static ref KEYSTATE_HELD: Mutex<HashSet<Scancode>> = Mutex::new(HashSet::new());
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::get_config_or_create());
}

fn pset(x: i32, y: i32, color: i32) {
    let idx = usize::from(TerminalLocation(x, y));
    if idx >= (WIDTH * HEIGHT) as usize {
        panic!(format!("Invalid pset coordinates X={}, Y={}", x, y));
    }

    PXBUF_MUTEX.lock().unwrap()[idx] = ColorPallete::from(color);
}

fn pget(x: i32, y: i32) -> i32 {
    let idx = usize::from(TerminalLocation(x, y));
    if idx >= (WIDTH * HEIGHT) as usize {
        panic!(format!("Invalid pget coordinates X={}, Y={}", x, y));
    }

    i32::from(PXBUF_MUTEX.lock().unwrap()[idx])
}

fn rect(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut fb = PXBUF_MUTEX.lock().unwrap();

    for x in x0..=x1 {
        fb[usize::from(TerminalLocation(x, y0))] = ColorPallete::from(color);
        fb[usize::from(TerminalLocation(x, y1))] = ColorPallete::from(color);
    }

    for y in y0..=y1 {
        fb[usize::from(TerminalLocation(x0, y))] = ColorPallete::from(color);
        fb[usize::from(TerminalLocation(x1, y))] = ColorPallete::from(color);
    }
}

fn rectfill(x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
    let mut fb = PXBUF_MUTEX.lock().unwrap();

    for x in x0..=x1 {
        for y in y0..=y1 {
            fb[usize::from(TerminalLocation(x, y))] = ColorPallete::from(color);
        }
    }
}

fn cls(color: i32) {
    rectfill(0, 0, WIDTH - 1, HEIGHT - 1, color);
}

fn key() -> i32 {
    let mut keystate_fifo = KEYSTATE_FRAME_FIFO.lock().unwrap();
    if keystate_fifo.len() > 0 {
        keystate_fifo.remove(0) as i32
    } else {
        0
    }
}

fn btn(i: i32, p: i32) -> i32 {
    let keystate_held = KEYSTATE_HELD.lock().unwrap();
    if p != 1 && p != 2 {
        panic!("Invalid player number {}", p);
    }

    let config = CONFIG.lock().unwrap();
    let keycode;
    if p == 1 {
        keycode = match i {
            0 => config.keys.player1.left,
            1 => config.keys.player1.right,
            2 => config.keys.player1.up,
            3 => config.keys.player1.down,
            4 => config.keys.player1.o,
            5 => config.keys.player1.x,
            _ => panic!("Invalid keycode {}", i),
        };
    } else {
        keycode = match i {
            0 => config.keys.player2.left,
            1 => config.keys.player2.right,
            2 => config.keys.player2.up,
            3 => config.keys.player2.down,
            4 => config.keys.player2.o,
            5 => config.keys.player2.x,
            _ => panic!("Invalid keycode {}", i),
        };
    }

    keystate_held.contains(&Scancode::from_i32(keycode).unwrap()) as i32
}

fn btnp(i: i32, p: i32) -> i32 {
    let keystate_frame = KEYSTATE_FRAME.lock().unwrap();
    if p != 1 && p != 2 {
        panic!("Invalid player number {}", p);
    }

    let config = CONFIG.lock().unwrap();
    let keycode;
    if p == 1 {
        keycode = match i {
            0 => config.keys.player1.left,
            1 => config.keys.player1.right,
            2 => config.keys.player1.up,
            3 => config.keys.player1.down,
            4 => config.keys.player1.o,
            5 => config.keys.player1.x,
            _ => panic!("Invalid keycode {}", i),
        };
    } else {
        keycode = match i {
            0 => config.keys.player2.left,
            1 => config.keys.player2.right,
            2 => config.keys.player2.up,
            3 => config.keys.player2.down,
            4 => config.keys.player2.o,
            5 => config.keys.player2.x,
            _ => panic!("Invalid keycode {}", i),
        };
    }

    keystate_frame.contains(&Scancode::from_i32(keycode).unwrap()) as i32
}

pub fn putc(c: u8, wx: i32, y: i32, col: i32) {
    if (c as usize) < BASIC_UNICODE.len() {
        let font = BASIC_UNICODE[c as usize];
        for row_offset in 0..8 {
            let row = font.byte_array()[row_offset];
            for bit in 0..8 {
                if (row >> (7 - bit)) & 1 == 1 {}
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
            let char = *ch;
            if (char as usize) < BASIC_UNICODE.len() {
                let font = BASIC_UNICODE[char as usize];
                for row_offset in 0u8..8 {
                    let row = font.byte_array()[row_offset as usize];
                    for bit in 0u8..8 {
                        if (row >> bit) & 1 == 1 {
                            let loc = TerminalLocation(
                                x + (offset * 8) + bit as i32,
                                y + row_offset as i32,
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

pub fn printh(caller: Caller, string_addr: i32) {
    println!(
        "{}",
        read_cstr(&WasmCallerWrapper::new(caller), string_addr)
    );
}

pub fn exit() {
    std::process::exit(0); // lol
}

fn main() {
    if std::env::args().len() == 1 {
        println!("Usage: {} <Binary>", std::env::args().next().unwrap());
        std::process::exit(1);
    }

    let mut path = "".to_owned();
    for x in std::env::args() {
        path = x;
    }

    // Setup SDL
    let sdl_ctx = sdl2::init().unwrap();
    let window = sdl_ctx
        .video()
        .unwrap()
        .window("WARS-8", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBX8888, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let binary_res = std::fs::read(path);
    if let Err(why) = binary_res {
        panic!("Failed to read binary! Reason: {}", why);
    }

    let mut runtime = WasmRuntime::new(&binary_res.unwrap()[..]);
    let mut import_vec: Vec<Extern> = Vec::new();
    let mut missing_import_vec: Vec<String> = Vec::new();
    for import in runtime.module.as_ref().unwrap().imports() {
        match import.name() {
            "exit" => import_vec.push(func_wrap!(runtime, exit)),
            "cls" => import_vec.push(func_wrap!(runtime, cls)),
            "pset" => import_vec.push(func_wrap!(runtime, pset)),
            "pget" => import_vec.push(func_wrap!(runtime, pget)),
            "rect" => import_vec.push(func_wrap!(runtime, rect)),
            "rectfill" => import_vec.push(func_wrap!(runtime, rectfill)),
            "btn" => import_vec.push(func_wrap!(runtime, btn)),
            "btnp" => import_vec.push(func_wrap!(runtime, btnp)),
            "print" => import_vec.push(func_wrap!(runtime, print)),
            "printh" => import_vec.push(func_wrap!(runtime, printh)),
            "key" => import_vec.push(func_wrap!(runtime, key)),
            _ => missing_import_vec.push(import.name().to_owned()),
        }
        println!("Attempting to import {}", import.name());
    }

    if missing_import_vec.len() != 0 {
        panic!(format!(
            "Missing {} imports: {:?}",
            missing_import_vec.len(),
            missing_import_vec
        ));
    }

    runtime.update_api(&import_vec[..]);

    let mut target_ms = sdl_ctx.timer().unwrap().ticks() + FRAME_LEN_MS;
    let mut fps_counter = FpsCounter::new(sdl_ctx.timer().unwrap().ticks());
    let mut paused = false;

    runtime.init();
    'sdlloop: loop {
        runtime.update();
        runtime.draw();

        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                let framebuffer = PXBUF_MUTEX.lock().unwrap();
                for idx in 0..framebuffer.len() {
                    let raw_idx = idx * 4;
                    let color = Color::from(framebuffer[idx]);
                    buffer[raw_idx + 1] = color.b;
                    buffer[raw_idx + 2] = color.g;
                    buffer[raw_idx + 3] = color.r;
                }
            })
            .unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        {
            let config = CONFIG.lock().unwrap();
            let mut keystate_frame = KEYSTATE_FRAME.lock().unwrap();
            let mut keystate_frame_fifo = KEYSTATE_FRAME_FIFO.lock().unwrap();
            keystate_frame.clear();
            keystate_frame_fifo.clear();
            let mut keystate_held = KEYSTATE_HELD.lock().unwrap();
            for event in sdl_ctx.event_pump().unwrap().poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'sdlloop;
                    }
                    Event::KeyDown {
                        scancode: Some(kc), ..
                    } if kc as i32 == config.keys.quit => {
                        break 'sdlloop;
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

                    _ => {}
                }
            }
        }

        let frame_difference = target_ms as i32 - sdl_ctx.timer().unwrap().ticks() as i32;
        target_ms += FRAME_LEN_MS;
        if frame_difference > 0 {
            sdl_ctx.timer().unwrap().delay(frame_difference as u32);
        }
        fps_counter.tick(sdl_ctx.timer().unwrap().ticks());
    }
}
