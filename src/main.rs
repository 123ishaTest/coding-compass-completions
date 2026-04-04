use std::collections::HashMap;
use std::fmt;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Currency {
    Coins,
    Diamonds,
}
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::Coins => write!(f, "Coins"),
            Currency::Diamonds => write!(f, "Diamonds"),
        }
    }
}

#[derive(Default)]
struct Wallet {
    balances: HashMap<Currency, i64>,
}
impl Wallet {
    fn gain_currency(&mut self, curr: Currency, amount: i64) {
        *self.balances.entry(curr).or_insert(0) += amount;
    }
    fn spend_currency(&mut self, curr: Currency, amount: i64) -> Result {
        *self.balances.entry(curr).or_insert(0) -= amount;
        Ok()
    }
    fn get_balance(&self, curr: Currency) -> String {
        let balance = self.balances.get(&curr).unwrap_or(&0);
        format!("{}: {}", curr.to_string(), balance)
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
        wallet.gain_currency(Currency::Coins, 0);
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&0));
        wallet.gain_currency(Currency::Coins, 4);
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&4));
        wallet.gain_currency(Currency::Coins, 3);
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&7));
    }
}
