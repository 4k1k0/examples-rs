use crate::bank::accounts::Account;

pub fn deposit(acc: &mut Account, amount: i32) {
    acc.balance += amount;
    println!(
        "[Transaction] Deposited: ${}. New balance: ${}",
        amount, acc.balance
    )
}

pub fn withdraw(acc: &mut Account, amount: i32) {
    if amount > acc.balance {
        println!(
            "[Transaction] Error: Insificient funds for ${} withdraw",
            amount
        )
    } else {
        acc.balance -= amount;
        println!(
            "[Transaction] withdrew: ${}. New balance: ${}",
            amount, acc.balance
        )
    }
}
