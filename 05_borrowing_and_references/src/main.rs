// References and Borrowing
// Safety and Performance where borrowing and references are powerful concepts.

// Understanding References
// References: allow you to refer to some value without taking ownership of it.
// Immutable References: &T
// Mutable References: &mut T

// fn main() {
//     // let _x = 5;
//     // let _r = &_x; // Immutable Reference

//     // *_r += 1; // Error: Cannot assign to immutable borrowed content

//     let mut _x = 5;
//     let _r = &mut _x; // Mutable Reference
//     *_r += 1;
//     *_r += -3;

//     println!("Value of _r: {}", _r);
//     println!("Value of _x: {}", _x);    
// }

fn main(){
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    
    // Mutable borrow to withdraw money
    account.withdraw(50.0);
    
    // Mutable borrow to deposit money
    account.deposit(100.0);
    
    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing ${} from account of {}", amount, self.owner);
        self.balance -= amount;

    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of ${}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${} to account of {}", amount, self.owner);
    }

    
}