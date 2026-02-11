pub mod accounts;
pub mod transactions;

pub fn announce(message: &str) {
    println!("[Bank announcement] {message}");
}
