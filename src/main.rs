const TEST_BINARY: &str = "../testapp/target/wasm32-unknown-unknown/debug/sysrs_8.wasm";

#[macro_use]
extern crate lazy_static;
extern crate wasmtime;
extern crate sdl2;

macro_rules! func_wrap {
    ($store:expr, $func:expr) => {
        Func::wrap(&$store, $func).into()
    };
}

mod fps_counter;
mod palette;

use crate::fps_counter::FpsCounter;
use crate::palette::ColorPallete;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use std::convert::From;
use std::collections::HashMap;
use std::sync::Mutex;
use wasmtime::*;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;

#[derive(Copy, Clone, Debug)]
pub struct TerminalLocation (pub i32, pub i32);

impl From<TerminalLocation> for usize {
    fn from(loc: TerminalLocation) -> Self {
        ((loc.1 * WIDTH) + loc.0) as usize
    }
}

lazy_static! {
    static ref PXBUF_MUTEX: Mutex<[ColorPallete; (WIDTH * HEIGHT) as usize]> = Mutex::new([ColorPallete::Black; (WIDTH * HEIGHT) as usize]);
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


fn main() {
    let mut API: HashMap<String, Func> = HashMap::new();
    // Setup SDL
    let sdl_ctx = sdl2::init().unwrap();
    let window = sdl_ctx.video().unwrap().window("WARS-8", WIDTH as u32, HEIGHT as u32).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBX8888, WIDTH as u32, HEIGHT as u32).unwrap();

    // Setup WASMTime
    let engine = Engine::new(Config::new().interruptable(true));
    let store = Store::new(&engine);
    let _interrupt_handle = store.interrupt_handle().unwrap();
    let module = Module::from_file(&engine, TEST_BINARY).unwrap();

    API.insert("pset".to_owned(), func_wrap!(store, pset));
    API.insert("pget".to_owned(), func_wrap!(store, pget));
    API.insert("rect".to_owned(), func_wrap!(store, rect));
    API.insert("rectfill".to_owned(), func_wrap!(store, rectfill));

    let mut import_vec: Vec<Extern> = Vec::new();
    let mut missing_import_vec: Vec<String> = Vec::new();
    for import in module.imports() {
        match import.name() {
            "pset" => import_vec.push(func_wrap!(store, pset)),
            "pget" => import_vec.push(func_wrap!(store, pget)),
            "rect" => import_vec.push(func_wrap!(store, rect)),
            "rectfill" => import_vec.push(func_wrap!(store, rectfill)),
            _ => missing_import_vec.push(import.name().to_owned()),
        }
        println!("Attempting to import {}", import.name());
    }

    if missing_import_vec.len() != 0 {
        panic!(format!("Missing {} imports: {:?}", missing_import_vec.len(), missing_import_vec));
    }

    let instance = Instance::new(&store, &module, &import_vec[..]).unwrap();
    let init_fn = instance.get_func("_init").expect("`_init` was not an exported function").get0::<()>().unwrap();
    let update_fn = instance.get_func("_update").expect("`_update` was not an exported function").get0::<()>().unwrap();
    let draw_fn = instance.get_func("_draw").expect("`_draw` was not an exported function").get0::<()>().unwrap();

    init_fn().unwrap();

    const TARGET_FPS: f32 = 30.0;
    const FRAME_LEN_MS: u32 = ((1.0 / TARGET_FPS) * 1000.0) as u32; 
    let mut target_ms = sdl_ctx.timer().unwrap().ticks() + FRAME_LEN_MS;

    let mut fps_counter = FpsCounter::new(sdl_ctx.timer().unwrap().ticks());
    
    'sdlloop: loop {
        for event in sdl_ctx.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'sdlloop
                },
                _ => { }
            }
        }

        update_fn().unwrap();
        draw_fn().unwrap();

        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            let framebuffer = PXBUF_MUTEX.lock().unwrap();
            for idx in 0..framebuffer.len() {
                let raw_idx = idx * 4;
                let color = Color::from(framebuffer[idx]);
                buffer[raw_idx + 1] = color.b;
                buffer[raw_idx + 2] = color.g;
                buffer[raw_idx + 3] = color.r;
            }
        }).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        let frame_difference = target_ms as i32 - sdl_ctx.timer().unwrap().ticks() as i32;
        target_ms += FRAME_LEN_MS;
        if frame_difference > 0 {
            sdl_ctx.timer().unwrap().delay(frame_difference as u32);
        }
        fps_counter.tick(sdl_ctx.timer().unwrap().ticks());
    }
}
