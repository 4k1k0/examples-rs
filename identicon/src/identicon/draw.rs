use image::{ImageBuffer, Rgb};

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

pub fn draw() {
    // a default (black) image containing Rgb values
    let mut image = ImageBuffer::<Rgb<u8>>::new(WIDTH, HEIGHT);

    // set a central pixel to white
    image.get_pixel_mut(5, 5).data = [255, 255, 255];

    // write it out to a file
    image.save("output.png").unwrap();
}
