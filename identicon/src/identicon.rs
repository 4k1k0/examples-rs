mod file;
mod name;
mod draw;

pub mod color;

pub fn run(s: &str) {
    let icon = name::new_icon(s);
    icon.info();
    draw::draw();
}
