use crate::identicon::color;
use crate::identicon::file;
use crate::identicon::grid;

#[derive(Debug)]
pub struct Icon {
    username: String,
    hex: [u8; 16],
    color: color::Color,
    filename: String,
    grid: [grid::Point; 25],
}

pub fn new_icon(username: &str) -> Icon {
    let hex = md5::compute(username).0;
    let color = color::new(hex);
    let username = username.to_string();
    let filename = file::generate_filename(hex);
    let grid = grid::new(hex);

    Icon {
        username,
        hex,
        color,
        filename,
        grid,
    }
}

impl Icon {
    pub fn info(&self) {
        println!("{:?}", self.hex);
        println!("{:?}", self.color);
        println!("{:?}", self.username);
        println!("{:?}", self.filename);
        println!("grid: {:?}", self.grid);

        self.color.info();
    }
}
