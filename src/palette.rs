use sdl2::pixels::Color;

#[derive(Copy, Clone, Debug)]
pub enum ColorPallete {
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

impl From<i32> for ColorPallete {
    fn from(color: i32) -> Self {
        match color {
            0 => ColorPallete::Black,
            1 => ColorPallete::DarkBlue,
            2 => ColorPallete::DarkPurple,
            3 => ColorPallete::DarkGreen,
            4 => ColorPallete::Brown,
            5 => ColorPallete::DarkGray,
            6 => ColorPallete::LightGray,
            7 => ColorPallete::White,
            8 => ColorPallete::Red,
            9 => ColorPallete::Orange,
            10 => ColorPallete::Yellow,
            11 => ColorPallete::Green,
            12 => ColorPallete::Blue,
            13 => ColorPallete::Indigo,
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
            ColorPallete::DarkPurple => 2,
            ColorPallete::DarkGreen => 3,
            ColorPallete::Brown => 4,
            ColorPallete::DarkGray => 5,
            ColorPallete::LightGray => 6,
            ColorPallete::White => 7,
            ColorPallete::Red => 8,
            ColorPallete::Orange => 9,
            ColorPallete::Yellow => 10,
            ColorPallete::Green => 11,
            ColorPallete::Blue => 12,
            ColorPallete::Indigo => 13,
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
            ColorPallete::DarkPurple => Color::from((0x7E, 0x25, 0x53)),
            ColorPallete::DarkGreen => Color::from((0x00, 0x87, 0x51)),
            ColorPallete::Brown => Color::from((0xAB, 0x52, 0x36)),
            ColorPallete::DarkGray => Color::from((0x5F, 0x57, 0x4F)),
            ColorPallete::LightGray => Color::from((0xC2, 0xC3, 0xC7)),
            ColorPallete::White => Color::from((0xFF, 0xF1, 0xE8)),
            ColorPallete::Red => Color::from((0xFF, 0x00, 0x4D)),
            ColorPallete::Orange => Color::from((0xFF, 0xA3, 0x00)),
            ColorPallete::Yellow => Color::from((0xFF, 0xEC, 0x27)),
            ColorPallete::Green => Color::from((0x00, 0xE4, 0x36)),
            ColorPallete::Blue => Color::from((0x29, 0xAD, 0xFF)),
            ColorPallete::Indigo => Color::from((0x83, 0x76, 0x9C)),
            ColorPallete::Pink => Color::from((0xFF, 0x77, 0xA8)),
            ColorPallete::Peach => Color::from((0xFF, 0xCC, 0xAA)),
        }
    }
}
