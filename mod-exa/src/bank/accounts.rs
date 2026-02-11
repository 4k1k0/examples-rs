#[derive(Debug)]
pub struct Account {
    pub owner: String,
    pub balance: i32,
}

impl Account {
    pub fn new(owner: &str) -> Self {
        Account {
            owner: String::from(owner),
            balance: 0,
        }
    }
}
