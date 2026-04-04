use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Currency {
    Coins,
    Diamonds,
}
impl Currency {}

#[derive(Default)]
struct Wallet {
    balances: HashMap<Currency, i64>,
}
impl Wallet {
    fn add_currency(&mut self, curr: Currency, amount: i64) {
        *self.balances.entry(curr).or_insert(0) += amount;
    }
}

fn main() {
    println!("Hello world!");
}

// ================ TESTS =====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_coins() {
        let mut wallet = Wallet::default();
        wallet.add_currency(Currency::Coins, 0);
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&0));
        wallet.add_currency(Currency::Coins, 4);
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&4));
        wallet.add_currency(Currency::Coins, 3);
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&7));
    }
}
