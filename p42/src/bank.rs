use std::fmt;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

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

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User: {}, Credit Line: {}, Balance: {}", self.name, self.credit_line, self.balance)
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank: {}, Credit Interest: {}bp, Debit Interest: {}bp", self.name, self.credit_interest, self.debit_interest)
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
                let positive = u64::try_from(user.balance).unwrap();
                liability = liability.checked_add(positive).unwrap();
            } else if user.balance < 0 {
                let neg = u64::try_from(user.balance.abs()).unwrap();
                asset = asset.checked_add(neg).unwrap();
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

        let amount_i64: i64 = amount.try_into()
            .map_err(|_| "Amount too large to process".to_string())?;

        {
            let from_user = self.users.get(from).unwrap();
            let new_balance = from_user.balance.checked_sub(amount_i64)
                .ok_or_else(|| "Arithmetic overflow in balance calculation".to_string())?;

            let credit_line_i64: i64 = from_user.credit_line.try_into()
                .map_err(|_| "Credit line too large to process".to_string())?;

            if new_balance < -credit_line_i64 {
                return Err("Insufficient credit line".to_string());
            }
        }

        let sender = self.users.get_mut(from).unwrap();
        sender.balance = sender.balance.checked_sub(amount_i64)
            .expect("Arithmetic overflow when deducting from sender");

        let receiver = self.users.get_mut(to).unwrap();
        if let Some(nb) = receiver.balance.checked_add(amount_i64) {
            receiver.balance = nb;
        } else {
            let rollback = self.users.get_mut(from).unwrap();
            rollback.balance = rollback.balance.checked_add(amount_i64).unwrap();
            return Err("Arithmetic overflow when adding to receiver".to_string());
        }

        Ok(())
    }

    pub fn accrue_interest(&mut self) -> Result<(), String> {
        for user in self.users.values_mut() {
            if user.balance < 0 {
                let abs_bal = u64::try_from(user.balance.abs())
                    .map_err(|_| format!("Balance too large for {}", user.name))?;
                let interest = abs_bal.checked_mul(self.credit_interest)
                    .ok_or_else(|| format!("Interest calc overflow for {}", user.name))? / 10000;
                let intr_i64 = i64::try_from(interest)
                    .map_err(|_| format!("Interest too large for {}", user.name))?;
                user.balance = user.balance.checked_sub(intr_i64)
                    .ok_or_else(|| format!("Balance underflow for {}", user.name))?;
            } else if user.balance > 0 {
                let pos = u64::try_from(user.balance)
                    .map_err(|_| format!("Balance too large for {}", user.name))?;
                let interest = pos.checked_mul(self.debit_interest)
                    .ok_or_else(|| format!("Interest calc overflow for {}", user.name))? / 10000;
                let intr_i64 = i64::try_from(interest)
                    .map_err(|_| format!("Interest too large for {}", user.name))?;
                user.balance = user.balance.checked_add(intr_i64)
                    .ok_or_else(|| format!("Balance overflow for {}", user.name))?;
            }
        }
        Ok(())
    }

    pub fn merge_bank(&mut self, other: Bank) -> Result<(), String> {
        for (n, u) in other.users {
            if let Some(eu) = self.users.get_mut(&n) {
                eu.balance = eu.balance.checked_add(u.balance)
                    .ok_or_else(|| format!("Balance overflow merging {}", n))?;
            } else {
                self.users.insert(n.clone(), u);
            }
        }
        self.credit_interest = self.credit_interest.checked_add(other.credit_interest)
            .ok_or_else(|| "Credit interest overflow".to_string())? / 2;
        self.debit_interest = self.debit_interest.checked_add(other.debit_interest)
            .ok_or_else(|| "Debit interest overflow".to_string())? / 2;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bank_creation() {
        let b = Bank::new("B".to_string(), 500, 300);
        assert_eq!(b.name, "B");
        assert_eq!(b.credit_interest, 500);
        assert_eq!(b.debit_interest, 300);
        assert!(b.users.is_empty());
    }
    #[test]
    fn test_add_user() {
        let mut b = Bank::new("B".to_string(), 500, 300);
        let u = User { name: "A".to_string(), credit_line: 1000, balance: 500 };
        assert!(b.add_user(u).is_ok());
        assert_eq!(b.users.len(), 1);
        assert!(b.add_user(User { name: "A".to_string(), credit_line: 0, balance: 0 }).is_err());
    }
    #[test]
    fn test_transfer_funds() {
        let mut b = Bank::new("B".to_string(), 0, 0);
        b.add_user(User { name: "A".to_string(), credit_line: 1000, balance: 500 }).unwrap();
        b.add_user(User { name: "C".to_string(), credit_line: 0, balance: 0 }).unwrap();
        assert!(b.transfer_funds("A", "C", 300).is_ok());
        assert_eq!(b.users.get("A").unwrap().balance, 200);
        assert_eq!(b.users.get("C").unwrap().balance, 300);
    }
}
