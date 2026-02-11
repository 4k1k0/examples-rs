mod bank;
mod merge;
mod validator;

fn main() {
    let mut acc = bank::accounts::Account::new("bob");
    println!("[Account] Created: {:?}", acc);

    bank::transactions::deposit(&mut acc, 150);
    bank::transactions::withdraw(&mut acc, 50);

    println!("[accounts] {} final state: {:?}",acc.owner, acc.balance);

    bank::announce("maintenance at 13:30pm");
    validator::foo();

    let is_mex_a = validator::country::is_mexico("USA");
    println!("USA is mex {}", is_mex_a);

    merge::run("wako");
}
