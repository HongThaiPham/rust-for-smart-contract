// Create a Trait called Account that defines the methods deposit, withdraw, and balance. These methods should take a mutable reference to self as an argument.
trait Account {
    fn deposit(&mut self, amount: u64) -> Result<(), String>;
    fn withdraw(&mut self, amount: u64) -> Result<(), String>;
    fn balance(&self) -> u64;
}

// Implement the Account Trait for a struct called BankAccount. The BankAccount struct should have the fields account_number, holder_name, and balance.
struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: u64,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: u64) -> Result<(), String> {
        if amount.eq(&0) {
            return Err("Deposit amount must be greater than 0".to_string());
        }
        self.balance += amount;
        Ok(())
    }
    fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if amount.eq(&0) {
            return Err("Withdraw amount must be greater than 0".to_string());
        }
        if self.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        self.balance -= amount;
        Ok(())
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

    // Call the deposit method on one of the accounts, passing in a deposit amount. Handle any errors returned by the deposit method using a match statement.
    let result1 = account1.deposit(0);
    match result1 {
        Ok(_) => println!("Deposit successful"),
        Err(e) => println!("Error: {}", e),
    }

    let result2 = account2.withdraw(50);
    match result2 {
        Ok(_) => println!("Withdraw successful"),
        Err(e) => println!("Error: {}", e),
    }

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
