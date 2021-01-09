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

// Api
mod api;

use crate::config::Config;
use crate::palette::ColorPallete;
use crate::runtime::wasm_runtime::WasmRuntime;
use crate::runtime::*;
use crate::utils::*;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashSet;
use std::convert::From;
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

lazy_static! {
    static ref PXBUF_MUTEX: Mutex<[ColorPallete; (WIDTH * HEIGHT) as usize]> =
        Mutex::new([ColorPallete::Black; (WIDTH * HEIGHT) as usize]);
    static ref KEYSTATE_FRAME: Mutex<HashSet<Scancode>> = Mutex::new(HashSet::new());
    static ref KEYSTATE_FRAME_FIFO: Mutex<Vec<Scancode>> = Mutex::new(Vec::new());
    static ref KEYSTATE_HELD: Mutex<HashSet<Scancode>> = Mutex::new(HashSet::new());
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::get_config_or_create());
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
            "cls" => import_vec.push(func_wrap!(runtime, api::gfx::cls)),
            "pset" => import_vec.push(func_wrap!(runtime, api::gfx::pset)),
            "pget" => import_vec.push(func_wrap!(runtime, api::gfx::pget)),
            "rect" => import_vec.push(func_wrap!(runtime, api::gfx::rect)),
            "rectfill" => import_vec.push(func_wrap!(runtime, api::gfx::rectfill)),
            "print" => import_vec.push(func_wrap!(runtime, api::gfx::print)),
            "printh" => import_vec.push(func_wrap!(runtime, api::gfx::printh)),

            "btn" => import_vec.push(func_wrap!(runtime, api::input::btn)),
            "btnp" => import_vec.push(func_wrap!(runtime, api::input::btnp)),
            "key" => import_vec.push(func_wrap!(runtime, api::input::key)),

            "exit" => import_vec.push(func_wrap!(runtime, api::misc::exit)),
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
