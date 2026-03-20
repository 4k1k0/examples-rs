mod identicon;

fn main() {
    let username = std::env::args().nth(1).expect("no username given");
    match identicon::run(&username) {
        Ok(_) => println!("success"),
        Err(e) => panic!("error: {}", e),
    }
}
