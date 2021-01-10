extern crate directories;
extern crate font8x8;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate rand_pcg;
extern crate sdl2;
extern crate serde_json;
extern crate wasmtime;

mod cart;
mod config;
mod palette;
mod runtime;
mod utils;

// Api
mod api;

use crate::cart::Cart;
use crate::cart::{wasm_binary::WasmBinary, wars_8_binary::Wars8Binary};
use crate::config::Config;
use crate::palette::ColorPallete;
use crate::runtime::wasm_runtime::WasmRuntime;
use crate::runtime::*;
use crate::utils::*;
use rand_pcg::Pcg64Mcg;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashSet;
use std::convert::From;
use std::sync::Mutex;

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
    static ref RAND_SRC: Mutex<Pcg64Mcg> = Mutex::new(Pcg64Mcg::new(0xcafef00dbeefd34d));
    static ref CART: Mutex<Option<Box<dyn Cart>>> = Mutex::new(None);
    static ref CART_TO_LOAD: Mutex<bool> = Mutex::new(false);
}

fn main() {
    let mut boot_cart_path;
    if std::env::args().len() == 1 {
        let path_obj = std::path::Path::new(&Config::get_config_dir_or_create().unwrap()).join("boot.wasm");
        boot_cart_path = path_obj.to_str().unwrap().to_string();
        if !path_obj.is_file() {
            println!("Usage: `{} <binary>`, or place a binary at `{}`", std::env::args().next().unwrap(), boot_cart_path);
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
        .window("WARS-8", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBX8888, WIDTH as u32, HEIGHT as u32)
        .unwrap();


    let mut cart_pre_mutex = CART.lock().unwrap();
    *cart_pre_mutex =  Some(Cart::load(&boot_cart_path));
    let mut runtime = WasmRuntime::new(cart_pre_mutex.as_deref().unwrap().binary());
    drop(cart_pre_mutex);
    runtime.init();

    let mut target_ms = sdl_ctx.timer().unwrap().ticks() + FRAME_LEN_MS;
    let mut fps_counter = FpsCounter::new(sdl_ctx.timer().unwrap().ticks());
    let mut paused = false;

    'sdlloop: loop {
        let mut cart_mutex = CART.lock().unwrap();
        let mut cart_to_load_mutex = CART_TO_LOAD.lock().unwrap();
        if cart_mutex.is_none() || *cart_to_load_mutex == true {
            *cart_to_load_mutex = false;
            if cart_mutex.is_none() {
                println!("Resetting to boot cartridge");
                *cart_mutex = Some(Cart::load(&boot_cart_path));
            }
            runtime = WasmRuntime::new(cart_mutex.as_deref().unwrap().binary());
            let mut framebuffer = PXBUF_MUTEX.lock().unwrap();
            for idx in 0..framebuffer.len() {
                framebuffer[idx] = ColorPallete::Black;
            }
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

                _ => {}
            }
        }

        drop(config);
        drop(keystate_held);
        drop(keystate_frame);
        drop(keystate_frame_fifo);

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

        let frame_difference = target_ms as i32 - sdl_ctx.timer().unwrap().ticks() as i32;
        target_ms += FRAME_LEN_MS;
        if frame_difference > 0 {
            sdl_ctx.timer().unwrap().delay(frame_difference as u32);
        }
        fps_counter.tick(sdl_ctx.timer().unwrap().ticks());
    }
}
