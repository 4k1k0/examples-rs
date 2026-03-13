use crate::identicon::name::new_icon;
mod draw;

pub mod color;
pub mod name;

pub fn run(s: &str) {
    let icon = new_icon(s);
    icon.info();
    draw::draw();
}
