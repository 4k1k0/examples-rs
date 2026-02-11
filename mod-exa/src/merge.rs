mod name;

pub fn run(name: &str) {
    name::filter_by_name(name);

    println!("hi {}", name)
}
