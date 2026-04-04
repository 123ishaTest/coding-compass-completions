use std::collections::HashMap;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Hash, PartialEq, Eq, Copy, Clone, EnumIter)]
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
impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletError::NegativeAmount => write!(f, "Negative Amount"),
            WalletError::InsufficientFunds => write!(f, "Insufficient Funds"),
        }
    }
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

    fn get_balance(&self, curr: Currency) -> i64 {
        return *self.balances.get(&curr).unwrap_or(&0);
    }

    fn gain_currencies(&mut self, gains: &[(Currency, i64)]) -> Result<(), WalletError> {
        for &(curr, amount) in gains {
            if amount < 0 {
                return Err(WalletError::NegativeAmount);
            }

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
    wallet_a: &mut Wallet,
    wallet_b: &mut Wallet,
) -> Result<(), TransactionError> {
    /*
     * Moves an amount of currency FROM wallet_a TO wallet_b, or error
     */
    if amount < 0 {
        return Err(TransactionError::NegativeAmount);
    }
    if wallet_a.get_balance(curr) < amount {
        return Err(TransactionError::InsufficientFunds);
    }

    wallet_a.spend_currency(curr, amount).unwrap();
    wallet_b.gain_currency(curr, amount).unwrap();

    Ok(())
}

fn merge_wallets(wallet_a: &Wallet, wallet_b: &Wallet) -> Wallet {
    /*
     * Creates a new wallet with the combined balances of both input wallets
     */
    let mut new_wallet = Wallet::default();

    for currency in Currency::iter() {
        let total = wallet_a.get_balance(currency) + wallet_b.get_balance(currency);
        if total > 0 {
            let _ = new_wallet.gain_currency(currency, total);
        }
    }

    new_wallet
}

fn main() {
    let mut wallet = Wallet::default();
    wallet.gain_currency(Currency::Coins, 10).unwrap();
    wallet.spend_currency(Currency::Coins, 3).unwrap();
    println!(
        "{}: {}",
        Currency::Coins.to_string(),
        wallet.get_balance(Currency::Coins)
    );

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

    #[test]
    fn test_transaction() {
        let mut wallet_a = Wallet::default();
        let mut wallet_b = Wallet::default();

        wallet_a.gain_currency(Currency::Coins, 5).unwrap();
        wallet_b.gain_currency(Currency::Coins, 5).unwrap();
        transaction(Currency::Coins, 3, &mut wallet_a, &mut wallet_b).unwrap();
        assert_eq!(wallet_a.get_balance(Currency::Coins), 2);
        assert_eq!(wallet_b.get_balance(Currency::Coins), 8);
    }

    #[test]
    fn test_merge() {
        let mut wallet_a = Wallet::default();
        let mut wallet_b = Wallet::default();
        wallet_a.gain_currency(Currency::Coins, 150).unwrap();
        wallet_a.gain_currency(Currency::Diamonds, 5).unwrap();
        wallet_b.gain_currency(Currency::Coins, 50).unwrap();

        let wallet_c = merge_wallets(&wallet_a, &wallet_b);

        assert_eq!(wallet_a.get_balance(Currency::Coins), 150);
        assert_eq!(wallet_a.get_balance(Currency::Diamonds), 5);
        assert_eq!(wallet_b.get_balance(Currency::Coins), 50);
        assert_eq!(wallet_c.get_balance(Currency::Coins), 200);
        assert_eq!(wallet_c.get_balance(Currency::Diamonds), 5);
    }
}
