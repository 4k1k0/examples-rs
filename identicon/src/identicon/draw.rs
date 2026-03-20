use crate::identicon::{color, pixel};
use image::{ImageBuffer, Rgb, RgbImage};

pub fn new(color: &color::Color, pixel_map: &Vec<pixel::PixelMap>) -> RgbImage {
    let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(250, 250, Rgb([255, 255, 255]));

    let pixel = Rgb([color.r, color.g, color.b]);

    for (point_a, point_b) in pixel_map {
        let x1 = point_a.0 as u32;
        let y1 = point_a.1 as u32;

        let x2 = point_b.0 as u32;
        let y2 = point_b.1 as u32;

        for x in x1..x2 {
            for y in y1..y2 {
                img.put_pixel(x, y, pixel);
            }
        }
    }

    img
}
