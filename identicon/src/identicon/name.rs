use crate::identicon::color;
use crate::identicon::file;

#[derive(Debug)]
pub struct Icon {
    username: String,
    hash: [u8; 16],
    color: color::Color,
    filename: String,
}

pub fn new_icon(username: &str) -> Icon {
    let hash = md5::compute(username).0;
    let color = color::new(hash);
    let username = username.to_string();
    let filename = file::generate_filename(hash);

    Icon {
        username,
        hash,
        color,
        filename,
    }
}

impl Icon {
    pub fn info(&self) {
        println!("{:?}", self.hash);
        println!("{:?}", self.color);
        println!("{:?}", self.username);
        println!("{:?}", self.filename);

        self.color.info();
    }
}
