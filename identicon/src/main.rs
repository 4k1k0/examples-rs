mod identicon;

fn main() {
    match identicon::run("wako") {
        Ok(_) => {
            println!("sucess");
        }
        Err(e) => {
            panic!("error: {}", e)
        }
    }
}
