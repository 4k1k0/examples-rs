mod identicon;

fn main() {
    match identicon::run("shushu") {
        Ok(_) => {
            println!("sucess");
        },
        Err(e) => {
            panic!("error: {}", e)
        }
        
    }
}
