use crate::identicon::name::new_icon;

pub mod color;
pub mod name;

pub fn run() {
    let icon = new_icon("wako");
    println!("{:?}",icon);
    name::foo(icon);
}
