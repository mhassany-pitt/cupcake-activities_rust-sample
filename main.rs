struct BankAccount {
    owner: String,
    balance: i32,
}

impl BankAccount {
    fn new(owner: &str, balance: i32) -> Self {
        Self {
            owner: owner.to_string(),
            balance,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        if amount > 0 {
            self.balance += amount;
            println!("Deposited {}", amount);
        }
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        if 0 < amount && amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew {}", amount);
        } else {
            println!("Insufficient funds");
        }
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new("Bob", 200);
    account.deposit(100);
    account.withdraw(50);
    println!("Balance: {}", account.balance);
}
