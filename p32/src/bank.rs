use std::fmt;
pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64
}

pub struct Bank {
    pub users: Vec<User>,
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
            name: name,
            users: Vec::new(),
            credit_interest,
            debit_interest,
        }
    }

    pub fn calc_balance(&self) -> (u64, u64) {
        let mut liability: u64 = 0;
        let mut asset: u64 = 0;

        for user in &self.users {
            if user.balance > 0 {
                liability += user.balance as u64;
            } else {
                asset += (-user.balance) as u64;
            }
        }

        (liability, asset)
    }

    pub fn transfer_funds(&mut self, from: String, to: String, amount: u64) -> Result<(), String> {

        let from_index = match self.users.iter().position(|u| u.name == from) {
            Some(idx) => idx,
            None => return Err(format!("User '{}' not found", from)),

        };

        let to_index = match self.users.iter().position(|u| u.name == to) {
            Some(idx) => idx,
            None => return Err(format!("User '{}' not found", from)),

        };

        if self.users[from_index].balance - (amount as i64) < -(self.users[from_index].credit_line as i64) {
            return Err(String::from("Insufficient credit line"));
        }

        self.users[from_index].balance -= amount as i64;
        self.users[to_index].balance += amount as i64;

        Ok(())
    }

    pub fn accrue_interest(&mut self) {
        for user in &mut self.users {
            if user.balance < 0 {
                let interest = (-(user.balance) as u64 * self.credit_interest) / 10000; // Convert basis points to percentage
                user.balance -= interest as i64;
            } else if user.balance > 0 {
                let interest = (user.balance as u64 * self.debit_interest) / 10000; // Convert basis points to percentage
                user.balance += interest as i64;
            }
        }
    }

    pub fn merge_bank(&mut self, other: Bank) {
        for user in other.users {
            if let Some(existing_user) = self.users.iter_mut().find(|u| u.name == user.name) {
                existing_user.balance += user.balance;
            } else {
                self.users.push(user);
            }
        }
        self.credit_interest = (self.credit_interest + other.credit_interest) / 2;
        self.debit_interest = (self.debit_interest + other.debit_interest) / 2;
    }
}
