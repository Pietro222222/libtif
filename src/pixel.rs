
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl std::convert::Into<u8> for PixelColor {
    fn into(self) -> u8 {
        match self {
            PixelColor::Black => 0x5b,
            PixelColor::Blue => 0x5a,
            PixelColor::Cyan => 0x61,
            PixelColor::Green => 0x5d,
            PixelColor::Magenta => 0x5e,
            PixelColor::Red => 0x5c,
            PixelColor::White => 0x5f,
            PixelColor::Yellow => 0x60,
        }
    }
}

impl std::convert::From<u8> for PixelColor {
    fn from(item: u8) -> Self {
         match item {
            90 => PixelColor::Blue,
            91 => PixelColor::Black,
            92 => PixelColor::Red,
            93 => PixelColor::Green,
            94 => PixelColor::Magenta,
            95 => PixelColor::White,
            96 => PixelColor::Yellow,
            97 => PixelColor::Cyan,
            _ => PixelColor::Black,
        }
    }
}

impl PixelColor {
    /// dont use this to convert to a tif color representation
    /// this is the correct way of doing so
    /// ```rs
    /// let tif_color_representation: u8 = pixel.into();
    /// ```
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
