// Create a Trait called Account that defines the methods deposit, withdraw, and balance. These methods should take a mutable reference to self as an argument.
trait Account {
    fn deposit(&mut self, amount: u64);
    fn withdraw(&mut self, amount: u64);
    fn balance(&self) -> u64;
}

// Implement the Account Trait for a struct called BankAccount. The BankAccount struct should have the fields account_number, holder_name, and balance.
struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: u64,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: u64) {
        self.balance -= amount;
    }
    fn balance(&self) -> u64 {
        self.balance
    }
}
fn main() {
    // create two BankAccount instances with different account numbers and holder names.
    let mut account1 = BankAccount {
        account_number: 1,
        holder_name: "Leo Pham".to_string(),
        balance: 1000,
    };

    let mut account2 = BankAccount {
        account_number: 2,
        holder_name: "Crush".to_string(),
        balance: 20,
    };

    account1.deposit(500);
    account2.withdraw(10);

    // Call the balance method on both accounts and print the result to the console.
    println!(
        "Account {}: {} balance: {}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "Account {}: {} balance: {}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}
