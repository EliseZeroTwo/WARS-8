use sdl2::pixels::Color;

#[derive(Copy, Clone, Debug)]
pub enum ColorPallete {
    Black,
    DarkBlue,
    Magenta,
    DarkGreen,
    Bronze,
    Grey,
    Silver,
    OffWhite,
    Red,
    Gold,
    Yellow,
    Green,
    SkyBlue,
    PalePurple,
    Pink,
    Peach,
}

impl From<i32> for ColorPallete {
    fn from(color: i32) -> Self {
        match color {
            0 => ColorPallete::Black,
            1 => ColorPallete::DarkBlue,
            2 => ColorPallete::Magenta,
            3 => ColorPallete::DarkGreen,
            4 => ColorPallete::Bronze,
            5 => ColorPallete::Grey,
            6 => ColorPallete::Silver,
            7 => ColorPallete::OffWhite,
            8 => ColorPallete::Red,
            9 => ColorPallete::Gold,
            10 => ColorPallete::Yellow,
            11 => ColorPallete::Green,
            12 => ColorPallete::SkyBlue,
            13 => ColorPallete::PalePurple,
            14 => ColorPallete::Pink,
            15 => ColorPallete::Peach,
            _ => panic!("Invalid color {}", color),
        }
    }
}

impl From<ColorPallete> for i32 {
    fn from(color: ColorPallete) -> Self {
        match color {
            ColorPallete::Black => 0,
            ColorPallete::DarkBlue => 1,
            ColorPallete::Magenta => 2,
            ColorPallete::DarkGreen => 3,
            ColorPallete::Bronze => 4,
            ColorPallete::Grey => 5,
            ColorPallete::Silver => 6,
            ColorPallete::OffWhite => 7,
            ColorPallete::Red => 8,
            ColorPallete::Gold => 9,
            ColorPallete::Yellow => 10,
            ColorPallete::Green => 11,
            ColorPallete::SkyBlue => 12,
            ColorPallete::PalePurple => 13,
            ColorPallete::Pink => 14,
            ColorPallete::Peach => 15,
        }
    }
}

impl From<ColorPallete> for Color {
    fn from(color: ColorPallete) -> Self {
        match color {
            ColorPallete::Black => Color::from((0x00, 0x00, 0x00)),
            ColorPallete::DarkBlue => Color::from((0x1D, 0x2B, 0x53)),
            ColorPallete::Magenta => Color::from((0x7E, 0x25, 0x53)),
            ColorPallete::DarkGreen => Color::from((0x00, 0x87, 0x51)),
            ColorPallete::Bronze => Color::from((0xAB, 0x52, 0x36)),
            ColorPallete::Grey => Color::from((0x5F, 0x57, 0x4F)),
            ColorPallete::Silver => Color::from((0xC2, 0xC3, 0xC7)),
            ColorPallete::OffWhite => Color::from((0xFF, 0xF1, 0xE8)),
            ColorPallete::Red => Color::from((0xFF, 0x00, 0x4D)),
            ColorPallete::Gold => Color::from((0xFF, 0xA3, 0x00)),
            ColorPallete::Yellow => Color::from((0xFF, 0xEC, 0x27)),
            ColorPallete::Green => Color::from((0x00, 0xE4, 0x36)),
            ColorPallete::SkyBlue => Color::from((0x29, 0xAD, 0xFF)),
            ColorPallete::PalePurple => Color::from((0x83, 0x76, 0x9C)),
            ColorPallete::Pink => Color::from((0xFF, 0x77, 0xA8)),
            ColorPallete::Peach => Color::from((0xFF, 0xCC, 0xAA)),
        }
    }
}
