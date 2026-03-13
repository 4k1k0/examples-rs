use crate::identicon::color::{Color, new};

#[derive(Debug)]
pub struct Icon {
    username: String,
    hash: [u8; 16],
    color: Color,
}

pub fn new_icon(username: &str) -> Icon {
    let hash = get_hash(username);
    let color = new(hash);
    let username = username.to_string();

    Icon {
        username,
        hash,
        color,
    }
}

pub fn get_hash(username: &str) -> [u8; 16] {
    md5::compute(username).0
}

impl Icon {
    pub fn info(&self) {
        println!("{:?}", self.hash);
        println!("{:?}", self.color);
        println!("{:?}", self.username);

        self.color.info();
    }
}
