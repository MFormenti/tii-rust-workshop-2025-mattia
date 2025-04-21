use std::fmt;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64
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
        write!(f, "User: {}, Credit Line: {}, Balance: {}",
               self.name, self.credit_line, self.balance)
    }
}

// For Bank
impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank: {}, Credit Interest: {}bp, Debit Interest: {}bp",
               self.name, self.credit_interest, self.debit_interest)
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
                // Convert positive i64 to u64 safely
                match u64::try_from(user.balance) {
                    Ok(positive_balance) => {
                        // Check for overflow when adding to liability
                        match liability.checked_add(positive_balance) {
                            Some(new_liability) => liability = new_liability,
                            None => {
                                // Handle overflow - in a real system, you might want a more sophisticated approach
                                println!("Warning: Liability calculation overflow");
                                liability = u64::MAX;
                            }
                        }
                    },
                    Err(_) => {
                        // This shouldn't happen since we're checking if balance > 0
                        println!("Warning: Unexpected error converting positive balance to u64");
                    }
                }
            } else if user.balance < 0 {
                // Convert negative i64 to u64 safely
                match u64::try_from(user.balance.abs()) {
                    Ok(negative_balance) => {
                        // Check for overflow when adding to asset
                        match asset.checked_add(negative_balance) {
                            Some(new_asset) => asset = new_asset,
                            None => {
                                // Handle overflow
                                println!("Warning: Asset calculation overflow");
                                asset = u64::MAX;
                            }
                        }
                    },
                    Err(_) => {
                        // This could happen if the abs() value is too large for u64
                        println!("Warning: Error converting negative balance abs value to u64");
                    }
                }
            }
        }

        (liability, asset)
    }

    pub fn transfer_funds(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        // Check if both users exist before making any changes
        if !self.users.contains_key(from) {
            return Err(format!("User '{}' not found", from));
        }

        if !self.users.contains_key(to) {
            return Err(format!("User '{}' not found", to));
        }

        // Convert amount to i64 safely
        let amount_i64: i64 = match amount.try_into() {
            Ok(val) => val,
            Err(_) => return Err("Amount too large to process".to_string()),
        };

        // Check credit line (we need to get the current balance and credit line first)
        {
            let from_user = self.users.get(from).unwrap(); // Safe unwrap as we checked earlier

            // Check if subtraction would exceed credit line
            let new_balance = match from_user.balance.checked_sub(amount_i64) {
                Some(balance) => balance,
                None => return Err("Arithmetic overflow in balance calculation".to_string()),
            };

            // Convert credit_line to i64 safely for comparison
            let credit_line_i64: i64 = match from_user.credit_line.try_into() {
                Ok(val) => val,
                Err(_) => return Err("Credit line too large to process".to_string()),
            };

            if new_balance < -credit_line_i64 {
                return Err("Insufficient credit line".to_string());
            }
        }

        // Now perform the transfers (using separate mutable borrows)
        // Deduct from sender
        if let Some(from_user) = self.users.get_mut(from) {
            from_user.balance = match from_user.balance.checked_sub(amount_i64) {
                Some(balance) => balance,
                None => return Err("Arithmetic overflow when deducting from sender".to_string()),
            };
        }

        // Add to receiver
        if let Some(to_user) = self.users.get_mut(to) {
            to_user.balance = match to_user.balance.checked_add(amount_i64) {
                Some(balance) => balance,
                None => {
                    // Rollback the deduction from sender since we hit an overflow
                    if let Some(from_user) = self.users.get_mut(from) {
                        from_user.balance += amount_i64; // Since we succeeded in deducting earlier, this should be safe
                    }
                    return Err("Arithmetic overflow when adding to receiver".to_string());
                },
            };
        }

        Ok(())
    }

    pub fn accrue_interest(&mut self) {
        for user in self.users.values_mut() {
            if user.balance < 0 {
                // Calculate interest for negative balance (debt)
                // First convert negative balance to positive u64 for calculation
                match u64::try_from(user.balance.abs()) {
                    Ok(abs_balance) => {
                        // Calculate interest
                        let interest = match abs_balance.checked_mul(self.credit_interest) {
                            Some(product) => product / 10000, // Convert basis points to percentage
                            None => {
                                println!("Warning: Interest calculation overflow for user {}", user.name);
                                continue; // Skip this interest calculation
                            }
                        };

                        // Convert interest back to i64 and deduct
                        match i64::try_from(interest) {
                            Ok(interest_i64) => {
                                user.balance = match user.balance.checked_sub(interest_i64) {
                                    Some(new_balance) => new_balance,
                                    None => {
                                        println!("Warning: Balance underflow when applying interest for user {}", user.name);
                                        i64::MIN // Set to minimum possible value
                                    }
                                };
                            },
                            Err(_) => {
                                println!("Warning: Interest too large to convert to i64 for user {}", user.name);
                            }
                        }
                    },
                    Err(_) => {
                        println!("Warning: Balance too large to calculate interest for user {}", user.name);
                    }
                }
            } else if user.balance > 0 {
                // Calculate interest for positive balance (savings)
                match u64::try_from(user.balance) {
                    Ok(positive_balance) => {
                        // Calculate interest
                        let interest = match positive_balance.checked_mul(self.debit_interest) {
                            Some(product) => product / 10000, // Convert basis points to percentage
                            None => {
                                println!("Warning: Interest calculation overflow for user {}", user.name);
                                continue; // Skip this interest calculation
                            }
                        };

                        // Convert interest back to i64 and add
                        match i64::try_from(interest) {
                            Ok(interest_i64) => {
                                user.balance = match user.balance.checked_add(interest_i64) {
                                    Some(new_balance) => new_balance,
                                    None => {
                                        println!("Warning: Balance overflow when applying interest for user {}", user.name);
                                        i64::MAX // Set to maximum possible value
                                    }
                                };
                            },
                            Err(_) => {
                                println!("Warning: Interest too large to convert to i64 for user {}", user.name);
                            }
                        }
                    },
                    Err(_) => {
                        println!("Warning: Balance too large to calculate interest for user {}", user.name);
                    }
                }
            }
        }
    }

    pub fn merge_bank(&mut self, other: Bank) {
        for (name, user) in other.users {
            if let Some(existing_user) = self.users.get_mut(&name) {
                // Add balances with overflow checking
                existing_user.balance = match existing_user.balance.checked_add(user.balance) {
                    Some(new_balance) => new_balance,
                    None => {
                        // Handle overflow
                        if user.balance > 0 {
                            i64::MAX
                        } else {
                            i64::MIN
                        }
                    }
                };
            } else {
                self.users.insert(name, user);
            }
        }

        // Average interest rates with overflow checking
        self.credit_interest = match self.credit_interest.checked_add(other.credit_interest) {
            Some(sum) => sum / 2,
            None => (u64::MAX / 2) + (if other.credit_interest % 2 == 1 { 1 } else { 0 }),
        };

        self.debit_interest = match self.debit_interest.checked_add(other.debit_interest) {
            Some(sum) => sum / 2,
            None => (u64::MAX / 2) + (if other.debit_interest % 2 == 1 { 1 } else { 0 }),
        };
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

        // Add user for the first time should succeed
        assert!(bank.add_user(user).is_ok());
        assert_eq!(bank.users.len(), 1);

        // Adding the same user should fail
        let duplicate_user = User {
            name: "Alice".to_string(),
            credit_line: 2000,
            balance: 1000,
        };
        assert!(bank.add_user(duplicate_user).is_err());

        // The original user should remain unchanged
        let alice = bank.users.get("Alice").unwrap();
        assert_eq!(alice.credit_line, 1000);
        assert_eq!(alice.balance, 500);
    }

    #[test]
    fn test_transfer_funds() {
        let mut bank = Bank::new("Test Bank".to_string(), 500, 300);

        // Add users
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

        // Test successful transfer
        assert!(bank.transfer_funds("Alice", "Bob", 300).is_ok());

        // Check balances after transfer
        assert_eq!(bank.users.get("Alice").unwrap().balance, 200);
        assert_eq!(bank.users.get("Bob").unwrap().balance, 500);

        // Test transfer exceeding credit line
        assert!(bank.transfer_funds("Alice", "Bob", 1500).is_err());

        // Balances should remain unchanged after failed transfer
        assert_eq!(bank.users.get("Alice").unwrap().balance, 200);
        assert_eq!(bank.users.get("Bob").unwrap().balance, 500);
    }

    #[test]
    fn test_accrue_interest() {
        let mut bank = Bank::new("Test Bank".to_string(), 500, 300); // 5% credit, 3% debit

        // Add users with different balance situations
        let alice = User {
            name: "Alice".to_string(),
            credit_line: 1000,
            balance: 1000, // Positive balance, will earn interest
        };
        let bob = User {
            name: "Bob".to_string(),
            credit_line: 2000,
            balance: -500, // Negative balance, will pay interest
        };

        bank.add_user(alice).unwrap();
        bank.add_user(bob).unwrap();

        // Apply interest
        bank.accrue_interest();

        // Check balances after interest
        // Alice: 1000 + (1000 * 0.03) = 1000 + 30 = 1030
        assert_eq!(bank.users.get("Alice").unwrap().balance, 1030);

        // Bob: -500 - (500 * 0.05) = -500 - 25 = -525
        assert_eq!(bank.users.get("Bob").unwrap().balance, -525);
    }

    #[test]
    fn test_merge_bank() {
        let mut bank1 = Bank::new("Bank 1".to_string(), 500, 300);
        let mut bank2 = Bank::new("Bank 2".to_string(), 600, 400);

        // Add users to first bank
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

        // Add users to second bank
        let charlie = User {
            name: "Charlie".to_string(),
            credit_line: 1500,
            balance: 800,
        };
        let alice_bank2 = User { // Same name as in bank1, will be merged
            name: "Alice".to_string(),
            credit_line: 500,
            balance: 500,
        };

        bank2.add_user(charlie).unwrap();
        bank2.add_user(alice_bank2).unwrap();

        // Merge banks
        bank1.merge_bank(bank2);

        // Check results
        assert_eq!(bank1.users.len(), 3); // Alice, Bob, Charlie
        assert_eq!(bank1.credit_interest, 550); // (500 + 600) / 2
        assert_eq!(bank1.debit_interest, 350); // (300 + 400) / 2

        // Check merged user balance
        assert_eq!(bank1.users.get("Alice").unwrap().balance, 1500); // 1000 + 500
        assert_eq!(bank1.users.get("Charlie").unwrap().balance, 800);
    }
}