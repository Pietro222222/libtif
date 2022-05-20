use crate::pixel::PixelColor;
use std::{fmt, io::Read};
const TIF_HEADER: [u8;5] = [46, 84, 73, 70, 32];

#[derive(Debug)]
pub enum TifImageError {
    InvalidHeader
}

impl fmt::Display for TifImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TifImageError{}

#[derive(Clone, Debug)]
pub struct TifImage {
   pub height: u64, //we dont really care about the type, height isnt defined inside of a tif file itself
   pub width: u8,
   pub pixels: Vec<Vec<PixelColor>>
}

impl TifImage {
    pub fn parse_from_file(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(filename)?;
        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        file.read_to_end(&mut buf)?;
        Self::parse_from_bytes(buf)
    }

    pub fn parse_from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        if bytes[0..5] != TIF_HEADER {
            return Err(Box::new(TifImageError::InvalidHeader));
        }
        //width is defined inside the tif file
        let width: u8 = bytes[5];

        //this are variables which we'll only use at parse-time
        let mut raw_pixels: Vec<PixelColor> = vec![];

        for byte in bytes[6..].chunks(2) {
            if byte[1] == 0 { continue; }
            for _ in 0..byte[1] {
                raw_pixels.push(PixelColor::from(byte[0]));
            }
        }

        //now we'll convert the raw pixels to the actual struct TifImage
        let height = (raw_pixels.len() as u64) / (width as u64);

        let pixels: Vec<Vec<PixelColor>> = raw_pixels.chunks(width as usize).map(|pixel| {
            pixel.to_vec()
        }).collect::<Vec<Vec<PixelColor>>>();

        Ok(Self {
            height,
            width,
            pixels
        })
    }

    pub fn save(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.extend(TIF_HEADER.iter());
        buf.push(self.width);
        let mut current_color = PixelColor::Black;
        let mut pixels: u8 = 0;

        for i in &self.pixels {
            for px in i {


                if *px == current_color {
                    pixels += 1;
                    if pixels >= 255 {
                        buf.push((*px).into());
                        buf.push(pixels);
                        pixels = 0;
                    }
                }else {
                    if pixels > 0 {
                        buf.push(current_color.into());
                        buf.push(pixels);
                    }
                    current_color = *px;
                    pixels = 1;
                }
            }
        }

        if pixels > 0 {
            buf.push(current_color.into());
            buf.push(pixels);
        }

        buf
    }

}
