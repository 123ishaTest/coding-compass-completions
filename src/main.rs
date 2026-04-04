use std::collections::HashMap;

enum Currency {
    coins,
    diamonds,
}
impl Currency {}

struct Wallet {
    balances: HashMap<Currency, i64>,
}
impl Wallet {}

fn main() {
    println!("Hello, world!");
}
