use std::{collections::HashMap, sync::MutexGuard};

use sdl2::pixels::Color;

use crate::draw_state;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ColorPalette {
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGray,
    LightGray,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Pink,
    Peach,
}

impl From<i32> for ColorPalette {
    fn from(color: i32) -> Self {
        match color {
            0 => ColorPalette::Black,
            1 => ColorPalette::DarkBlue,
            2 => ColorPalette::DarkPurple,
            3 => ColorPalette::DarkGreen,
            4 => ColorPalette::Brown,
            5 => ColorPalette::DarkGray,
            6 => ColorPalette::LightGray,
            7 => ColorPalette::White,
            8 => ColorPalette::Red,
            9 => ColorPalette::Orange,
            10 => ColorPalette::Yellow,
            11 => ColorPalette::Green,
            12 => ColorPalette::Blue,
            13 => ColorPalette::Indigo,
            14 => ColorPalette::Pink,
            15 => ColorPalette::Peach,
            _ => panic!("Invalid color {}", color),
        }
    }
}

impl From<ColorPalette> for i32 {
    fn from(color: ColorPalette) -> Self {
        match color {
            ColorPalette::Black => 0,
            ColorPalette::DarkBlue => 1,
            ColorPalette::DarkPurple => 2,
            ColorPalette::DarkGreen => 3,
            ColorPalette::Brown => 4,
            ColorPalette::DarkGray => 5,
            ColorPalette::LightGray => 6,
            ColorPalette::White => 7,
            ColorPalette::Red => 8,
            ColorPalette::Orange => 9,
            ColorPalette::Yellow => 10,
            ColorPalette::Green => 11,
            ColorPalette::Blue => 12,
            ColorPalette::Indigo => 13,
            ColorPalette::Pink => 14,
            ColorPalette::Peach => 15,
        }
    }
}

impl From<ColorPalette> for Color {
    fn from(color: ColorPalette) -> Self {
        match color {
            ColorPalette::Black => Color::from((0x00, 0x00, 0x00)),
            ColorPalette::DarkBlue => Color::from((0x1D, 0x2B, 0x53)),
            ColorPalette::DarkPurple => Color::from((0x7E, 0x25, 0x53)),
            ColorPalette::DarkGreen => Color::from((0x00, 0x87, 0x51)),
            ColorPalette::Brown => Color::from((0xAB, 0x52, 0x36)),
            ColorPalette::DarkGray => Color::from((0x5F, 0x57, 0x4F)),
            ColorPalette::LightGray => Color::from((0xC2, 0xC3, 0xC7)),
            ColorPalette::White => Color::from((0xFF, 0xF1, 0xE8)),
            ColorPalette::Red => Color::from((0xFF, 0x00, 0x4D)),
            ColorPalette::Orange => Color::from((0xFF, 0xA3, 0x00)),
            ColorPalette::Yellow => Color::from((0xFF, 0xEC, 0x27)),
            ColorPalette::Green => Color::from((0x00, 0xE4, 0x36)),
            ColorPalette::Blue => Color::from((0x29, 0xAD, 0xFF)),
            ColorPalette::Indigo => Color::from((0x83, 0x76, 0x9C)),
            ColorPalette::Pink => Color::from((0xFF, 0x77, 0xA8)),
            ColorPalette::Peach => Color::from((0xFF, 0xCC, 0xAA)),
        }
    }
}

impl ColorPalette {
    pub fn apply_palette_mod(
        self,
        mutex_guard: Option<&MutexGuard<[u8; 0x8000]>>,
        screen: bool,
    ) -> ColorPalette {
        if screen {
            draw_state::get_screen_palette(mutex_guard, self)
        } else {
            draw_state::get_draw_palette(mutex_guard, self).0
        }
    }
}
