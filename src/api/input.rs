use crate::{CONFIG, KEYSTATE_FRAME, KEYSTATE_FRAME_FIFO, KEYSTATE_HELD};
use sdl2::keyboard::Scancode;

pub fn key() -> i32 {
    let mut keystate_fifo = KEYSTATE_FRAME_FIFO.lock().unwrap();
    if keystate_fifo.len() > 0 {
        keystate_fifo.remove(0) as i32
    } else {
        0
    }
}
pub fn btn(i: i32, p: i32) -> i32 {
    let keystate_held = KEYSTATE_HELD.lock().unwrap();
    if p != 0 && p != 1 {
        panic!("Invalid player number {}", p);
    }

    let config = CONFIG.lock().unwrap();
    let keycode;
    if p == 0 {
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
pub fn btnp(i: i32, p: i32) -> i32 {
    let keystate_frame = KEYSTATE_FRAME.lock().unwrap();
    if p != 0 && p != 1 {
        panic!("Invalid player number {}", p);
    }

    let config = CONFIG.lock().unwrap();
    let keycode;
    if p == 0 {
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
