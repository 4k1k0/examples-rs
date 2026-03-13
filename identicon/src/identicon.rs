mod draw;
mod file;
mod name;

pub mod color;
pub mod grid;

pub fn run(s: &str) {
    let icon = name::new_icon(s);
    icon.info();
    draw::draw();
}
