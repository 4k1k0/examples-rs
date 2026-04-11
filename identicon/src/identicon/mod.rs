mod draw;
mod file;
mod name;
mod pixel;

pub mod color;
pub mod grid;

pub fn run(s: &str) -> Result<(), image::ImageError> {
    let icon = name::new_icon(s);
    icon.info();

    icon.save()
}
