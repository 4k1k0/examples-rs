#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn new(hash: [u8; 16]) -> Color {
    Color {
        r: hash[0],
        g: hash[1],
        b: hash[2],
    }
}

impl Color {
    pub fn info(&self) {
        println!("R:{} | G:{} | B:{}", self.r, &self.g, &self.b)
    }
}
