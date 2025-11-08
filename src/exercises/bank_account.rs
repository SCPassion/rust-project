fn main() {
    let mut account1 = BankAccount {
        _account_number: 12345,
        holder_name: String::from("Wong"),
        balance: 0,
    };

    let mut account2 = BankAccount {
        _account_number: 23456,
        holder_name: String::from("SCP"),
        balance: 10,
    };

    println!("{} has {} before", account1.holder_name, account1.balance());
    println!("{} has {} before", account2.holder_name, account2.balance());

    account1.deposit(5);
    account2.withdraw(1);

    println!("{} has {} after", account1.holder_name, account1.balance());
    println!("{} has {} after", account2.holder_name, account2.balance());
}

trait Account {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
    fn balance(&self) -> u32;
}

struct BankAccount {
    _account_number: u32,
    holder_name: String,
    balance: u32,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u32) {
        self.balance -= amount;
    }

    fn balance(&self) -> u32 {
        self.balance
    }
}
