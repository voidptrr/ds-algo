// https://leetcode.com/problems/simple-bank-system

pub struct Bank {
    balances: Vec<i64>,
}

impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        Bank { balances: balance }
    }

    fn is_valid_index(&self, account: i32) -> bool {
        (1..=self.balances.len() as i32).contains(&account)
    }

    pub fn transfer(
        &mut self,
        account1: i32,
        account2: i32,
        money: i64,
    ) -> bool {
        if !self.is_valid_index(account1) || !self.is_valid_index(account2) {
            return false;
        }

        let from_balance_index = (account1 - 1) as usize;
        let to_balance_index = (account2 - 1) as usize;

        if money > self.balances[from_balance_index] {
            return false;
        }

        self.balances[from_balance_index] -= money;
        self.balances[to_balance_index] += money;

        true
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.is_valid_index(account) {
            return false;
        }

        let balance_index = (account - 1) as usize;
        self.balances[balance_index] += money;

        true
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.is_valid_index(account) {
            return false;
        }

        let index = (account - 1) as usize;

        if money > self.balances[index] {
            return false;
        }

        self.balances[index] -= money;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Bank;

    #[test]
    fn example_one() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);

        assert!(bank.withdraw(3, 10));
        assert!(bank.transfer(5, 1, 20));
        assert!(bank.deposit(5, 20));
        assert!(!bank.transfer(3, 4, 15));
        assert!(!bank.withdraw(10, 50));
    }
}
