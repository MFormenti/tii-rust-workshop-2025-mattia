use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;

pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64,
}

pub struct Bank {
    pub users: HashMap<String, User>,
    pub name: String,
    pub credit_interest: u64,
    pub debit_interest: u64,
}

// For User
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User: {}, Credit Line: {}, Balance: {}",
            self.name, self.credit_line, self.balance
        )
    }
}

// For Bank
impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bank: {}, Credit Interest: {}bp, Debit Interest: {}bp",
            self.name, self.credit_interest, self.debit_interest
        )
    }
}

impl Bank {
    pub fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Bank {
            name,
            users: HashMap::new(),
            credit_interest,
            debit_interest,
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.name) {
            return Err(format!("User '{}' already exists", user.name));
        }

        self.users.insert(user.name.clone(), user);
        Ok(())
    }

    pub fn calc_balance(&self) -> (u64, u64) {
        let mut liability: u64 = 0;
        let mut asset: u64 = 0;

        for user in self.users.values() {
            if user.balance > 0 {
                let positive_balance =
                    u64::try_from(user.balance).expect("Positive balance conversion to u64 failed");

                liability = liability
                    .checked_add(positive_balance)
                    .expect("Liability calculation overflow");
            } else if user.balance < 0 {
                let negative_balance = u64::try_from(user.balance.abs())
                    .expect("Negative balance conversion to u64 failed");

                asset = asset
                    .checked_add(negative_balance)
                    .expect("Asset calculation overflow");
            }
        }

        (liability, asset)
    }

    pub fn transfer_funds(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        if !self.users.contains_key(from) {
            return Err(format!("User '{}' not found", from));
        }

        if !self.users.contains_key(to) {
            return Err(format!("User '{}' not found", to));
        }

        let amount_i64: i64 = amount
            .try_into()
            .map_err(|_| "Amount too large to process".to_string())?;

        {
            let from_user = self.users.get(from).unwrap();

            let Some(new_balance) = from_user.balance.checked_sub(amount_i64) else {
                return Err("Arithmetic overflow in balance calculation".to_string());
            };

            let credit_line_i64: i64 = from_user
                .credit_line
                .try_into()
                .map_err(|_| "Credit line too large to process".to_string())?;

            if new_balance < -credit_line_i64 {
                return Err("Insufficient credit line".to_string());
            }
        }

        if let Some(from_user) = self.users.get_mut(from) {
            from_user.balance = from_user
                .balance
                .checked_sub(amount_i64)
                .expect("Arithmetic overflow when deducting from sender");
        }

        if let Some(to_user) = self.users.get_mut(to) {
            to_user.balance = match to_user.balance.checked_add(amount_i64) {
                Some(balance) => balance,
                None => {
                    if let Some(from_user) = self.users.get_mut(from) {
                        from_user.balance += amount_i64;
                    }
                    return Err("Arithmetic overflow when adding to receiver".to_string());
                }
            };
        }

        Ok(())
    }

    pub fn accrue_interest(&mut self) -> Result<(), String> {
        for user in self.users.values_mut() {
            match user.balance.cmp(&0) {
                std::cmp::Ordering::Less => {
                    let abs_balance = u64::try_from(user.balance.abs()).map_err(|_| {
                        format!(
                            "Balance too large to calculate interest for user {}",
                            user.name
                        )
                    })?;

                    let interest = match abs_balance.checked_mul(self.credit_interest) {
                        Some(result) => result / 10000,
                        None => {
                            return Err(format!(
                                "Interest calculation overflow for user {}",
                                user.name
                            ));
                        }
                    };

                    let interest_i64 = i64::try_from(interest).map_err(|_| {
                        format!(
                            "Interest too large to convert to i64 for user {}",
                            user.name
                        )
                    })?;

                    user.balance = match user.balance.checked_sub(interest_i64) {
                        Some(result) => result,
                        None => {
                            return Err(format!(
                                "Balance underflow when applying interest for user {}",
                                user.name
                            ));
                        }
                    };
                }
                std::cmp::Ordering::Greater => {
                    let positive_balance = u64::try_from(user.balance).map_err(|_| {
                        format!(
                            "Balance too large to calculate interest for user {}",
                            user.name
                        )
                    })?;

                    let interest = match positive_balance.checked_mul(self.debit_interest) {
                        Some(result) => result / 10000,
                        None => {
                            return Err(format!(
                                "Interest calculation overflow for user {}",
                                user.name
                            ));
                        }
                    };

                    let interest_i64 = i64::try_from(interest).map_err(|_| {
                        format!(
                            "Interest too large to convert to i64 for user {}",
                            user.name
                        )
                    })?;

                    user.balance = match user.balance.checked_add(interest_i64) {
                        Some(result) => result,
                        None => {
                            return Err(format!(
                                "Balance overflow when applying interest for user {}",
                                user.name
                            ));
                        }
                    };
                }
                std::cmp::Ordering::Equal => {
                    // No interest for zero balance
                }
            }
        }

        Ok(())
    }

    pub fn merge_bank(&mut self, other: Bank) -> Result<(), String> {
        for (name, user) in other.users {
            if let Some(existing_user) = self.users.get_mut(&name) {
                existing_user.balance = match existing_user.balance.checked_add(user.balance) {
                    Some(result) => result,
                    None => return Err(format!("Balance overflow when merging user {}", name)),
                };
            } else {
                self.users.insert(name, user);
            }
        }

        self.credit_interest = match self.credit_interest.checked_add(other.credit_interest) {
            Some(result) => result / 2,
            None => return Err("Credit interest overflow during merge".to_string()),
        };

        self.debit_interest = match self.debit_interest.checked_add(other.debit_interest) {
            Some(result) => result / 2,
            None => return Err("Debit interest overflow during merge".to_string()),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_creation() {
        let bank = Bank::new("Test Bank".to_string(), 500, 300);
        assert_eq!(bank.name, "Test Bank");
        assert_eq!(bank.credit_interest, 500);
        assert_eq!(bank.debit_interest, 300);
        assert!(bank.users.is_empty());
    }

    #[test]
    fn test_add_user() {
        let mut bank = Bank::new("Test Bank".to_string(), 500, 300);
        let user = User {
            name: "Alice".to_string(),
            credit_line: 1000,
            balance: 500,
        };

        assert!(bank.add_user(user).is_ok());
        assert_eq!(bank.users.len(), 1);

        let duplicate_user = User {
            name: "Alice".to_string(),
            credit_line: 2000,
            balance: 1000,
        };
        assert!(bank.add_user(duplicate_user).is_err());

        let alice = bank.users.get("Alice").unwrap();
        assert_eq!(alice.credit_line, 1000);
        assert_eq!(alice.balance, 500);
    }

    #[test]
    fn test_transfer_funds() {
        let mut bank = Bank::new("Test Bank".to_string(), 500, 300);

        let alice = User {
            name: "Alice".to_string(),
            credit_line: 1000,
            balance: 500,
        };
        let bob = User {
            name: "Bob".to_string(),
            credit_line: 2000,
            balance: 200,
        };

        bank.add_user(alice).unwrap();
        bank.add_user(bob).unwrap();

        assert!(bank.transfer_funds("Alice", "Bob", 300).is_ok());

        assert_eq!(bank.users.get("Alice").unwrap().balance, 200);
        assert_eq!(bank.users.get("Bob").unwrap().balance, 500);

        assert!(bank.transfer_funds("Alice", "Bob", 1500).is_err());

        assert_eq!(bank.users.get("Alice").unwrap().balance, 200);
        assert_eq!(bank.users.get("Bob").unwrap().balance, 500);
    }

    #[test]
    fn test_accrue_interest() {
        let mut bank = Bank::new("Test Bank".to_string(), 500, 300);

        let alice = User {
            name: "Alice".to_string(),
            credit_line: 1000,
            balance: 1000,
        };
        let bob = User {
            name: "Bob".to_string(),
            credit_line: 2000,
            balance: -500,
        };

        bank.add_user(alice).unwrap();
        bank.add_user(bob).unwrap();

        assert!(bank.accrue_interest().is_ok());

        assert_eq!(bank.users.get("Alice").unwrap().balance, 1030);
        assert_eq!(bank.users.get("Bob").unwrap().balance, -525);
    }

    #[test]
    fn test_merge_bank() {
        let mut bank1 = Bank::new("Bank 1".to_string(), 500, 300);
        let mut bank2 = Bank::new("Bank 2".to_string(), 600, 400);

        let alice = User {
            name: "Alice".to_string(),
            credit_line: 1000,
            balance: 1000,
        };
        let bob = User {
            name: "Bob".to_string(),
            credit_line: 2000,
            balance: -500,
        };

        bank1.add_user(alice).unwrap();
        bank1.add_user(bob).unwrap();

        let charlie = User {
            name: "Charlie".to_string(),
            credit_line: 1500,
            balance: 800,
        };
        let alice_bank2 = User {
            name: "Alice".to_string(),
            credit_line: 500,
            balance: 500,
        };

        bank2.add_user(charlie).unwrap();
        bank2.add_user(alice_bank2).unwrap();

        assert!(bank1.merge_bank(bank2).is_ok());

        assert_eq!(bank1.users.len(), 3);
        assert_eq!(bank1.credit_interest, 550);
        assert_eq!(bank1.debit_interest, 350);

        assert_eq!(bank1.users.get("Alice").unwrap().balance, 1500);
        assert_eq!(bank1.users.get("Charlie").unwrap().balance, 800);
    }
}
