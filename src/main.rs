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

#[derive(Debug, PartialEq)]
enum WalletError {
    NegativeAmount,
    InsufficientFunds,
}

#[derive(Default)]
struct Wallet {
    balances: HashMap<Currency, i64>,
}
impl Wallet {
    fn gain_currency(&mut self, curr: Currency, amount: i64) -> Result<(), WalletError> {
        if amount < 0 {
            return Err(WalletError::NegativeAmount);
        }

        *self.balances.entry(curr).or_insert(0) += amount;
        Ok(())
    }

    fn spend_currency(&mut self, curr: Currency, amount: i64) -> Result<(), WalletError> {
        if amount < 0 {
            return Err(WalletError::NegativeAmount);
        }

        let balance = self.balances.entry(curr).or_insert(0);

        if *balance < amount {
            return Err(WalletError::InsufficientFunds);
        }

        *balance -= amount;
        Ok(())
    }

    fn get_balance(&self, curr: Currency) -> String {
        let balance = self.balances.get(&curr).unwrap_or(&0);
        format!("{}: {}", curr.to_string(), balance)
    }

    fn gain_currencies(&mut self, gains: &[(Currency, i64)]) -> Result<(), WalletError> {
        for &(curr, amount) in gains {
            if amount < 0 {
                return Err(WalletError::NegativeAmount);
            }
        }

        for &(curr, amount) in gains {
            *self.balances.entry(curr).or_insert(0) += amount;
        }

        Ok(())
    }
    fn spend_currencies(&mut self, spends: &[(Currency, i64)]) -> Result<(), WalletError> {
        for &(_, amount) in spends {
            if amount < 0 {
                return Err(WalletError::NegativeAmount);
            }
        }

        // Sum duplicates first
        let mut totals: HashMap<Currency, i64> = HashMap::new();
        for &(curr, amount) in spends {
            *totals.entry(curr).or_insert(0) += amount;
        }

        // Validate all
        for (curr, total) in &totals {
            let balance = *self.balances.get(curr).unwrap_or(&0);
            if balance < *total {
                return Err(WalletError::InsufficientFunds);
            }
        }

        // Apply only if all valid
        for (curr, total) in totals {
            *self.balances.entry(curr).or_insert(0) -= total;
        }

        Ok(())
    }
    fn get_balances(&self) -> Vec<(Currency, i64)> {
        self.balances
            .iter()
            .map(|(curr, amount)| (*curr, *amount))
            .collect()
    }
}

#[derive(Debug, PartialEq)]
enum TransactionError {
    NegativeAmount,
    InsufficientFunds,
}

// Perhaps this logic should belong to a manager
fn transaction(
    curr: Currency,
    amount: i64,
    &mut walletA: Wallet,
    &mut walletB: Wallet,
) -> Result<(), TransactionError> {
    /*
    * Moves an amount of currency FROM walletA TO walletB, or error
    */
}

fn main() {
    let mut wallet = Wallet::default();
    wallet.gain_currency(Currency::Coins, 10).unwrap();
    wallet.spend_currency(Currency::Coins, 3).unwrap();
    println!("{}", wallet.get_balance(Currency::Coins));

    // Print different currency
    wallet.gain_currency(Currency::Diamonds, 1).unwrap();
    println!("{}", wallet.get_balance(Currency::Diamonds));

    // Print multiple
    let balances = wallet.get_balances();
    for (currency, amount) in balances {
        println!("{}: {}", currency, amount);
    }
}

// ================ TESTS ====================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_coins() {
        let mut wallet = Wallet::default();
        assert_eq!(wallet.gain_currency(Currency::Coins, 0), Ok(()));
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&0));
        assert_eq!(wallet.gain_currency(Currency::Coins, 4), Ok(()));
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&4));
        assert_eq!(wallet.gain_currency(Currency::Coins, 3), Ok(()));
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&7));
    }

    #[test]
    fn test_gain_different_currencies() {
        let mut wallet = Wallet::default();
        assert_eq!(wallet.gain_currency(Currency::Coins, 5), Ok(()));
        assert_eq!(wallet.gain_currency(Currency::Diamonds, 2), Ok(()));
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&5));
        assert_eq!(wallet.balances.get(&Currency::Diamonds), Some(&2));
    }

    #[test]
    fn test_spend_coins() {
        let mut wallet = Wallet::default();
        wallet.gain_currency(Currency::Coins, 10).unwrap();
        assert_eq!(wallet.spend_currency(Currency::Coins, 4), Ok(()));
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&6));
    }

    #[test]
    fn test_overspend_coins() {
        let mut wallet = Wallet::default();
        wallet.gain_currency(Currency::Coins, 3).unwrap();
        assert_eq!(
            wallet.spend_currency(Currency::Coins, 5),
            Err(WalletError::InsufficientFunds)
        );
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&3));
    }

    #[test]
    fn test_negative_amounts() {
        let mut wallet = Wallet::default();
        assert_eq!(
            wallet.gain_currency(Currency::Coins, -5),
            Err(WalletError::NegativeAmount)
        );
        assert_eq!(
            wallet.gain_currency(Currency::Diamonds, -5),
            Err(WalletError::NegativeAmount)
        );
        assert_eq!(
            wallet.spend_currency(Currency::Coins, -5),
            Err(WalletError::NegativeAmount)
        );
        assert_eq!(
            wallet.spend_currency(Currency::Diamonds, -5),
            Err(WalletError::NegativeAmount)
        );
    }

    #[test]
    fn test_gain_currencies() {
        let mut wallet = Wallet::default();
        let gains = [(Currency::Coins, 5), (Currency::Diamonds, 2)];
        assert_eq!(wallet.gain_currencies(&gains), Ok(()));
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&5));
        assert_eq!(wallet.balances.get(&Currency::Diamonds), Some(&2));
    }

    #[test]
    fn test_spend_currencies_partial_insufficient() {
        let mut wallet = Wallet::default();
        wallet.gain_currency(Currency::Coins, 5).unwrap();
        wallet.gain_currency(Currency::Diamonds, 1).unwrap();
        let spends = [(Currency::Coins, 4), (Currency::Diamonds, 2)];
        assert_eq!(
            wallet.spend_currencies(&spends),
            Err(WalletError::InsufficientFunds)
        );
        // Atomic failure, no change
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&5));
        assert_eq!(wallet.balances.get(&Currency::Diamonds), Some(&1));
    }

    #[test]
    fn test_spend_duplicate_currencies_total_insufficient() {
        let mut wallet = Wallet::default();
        wallet.gain_currency(Currency::Coins, 5).unwrap();
        let spends = [(Currency::Coins, 2), (Currency::Coins, 4)];
        assert_eq!(
            wallet.spend_currencies(&spends),
            Err(WalletError::InsufficientFunds)
        );
        // No change
        assert_eq!(wallet.balances.get(&Currency::Coins), Some(&5));
    }
}
