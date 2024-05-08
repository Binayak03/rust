use std::io;

fn main() {
    let mut account1 = depositor();
    
    let mut account2 = receiver();

    account1.deposit(400.0);
    account2.withdraw(400.0);

   

    fn depositor() -> BankAccount {
        println!("Enter Depositor account number:");
        let mut account_number = String::new();
        io::stdin().read_line(&mut account_number).expect("Failed to read line");
        let account_number: u32 = account_number.trim().parse().expect("Please enter a number");
    
        println!("Enter Depositor name:");
        let mut holder_name = String::new();
        io::stdin().read_line(&mut holder_name).expect("Failed to read line");
        let holder_name = holder_name.trim().to_string();
    
        BankAccount {
            account_number,
            holder_name,
            balance: 1000.0,
        }


    }

    fn receiver() -> BankAccount {
        println!("Enter Receiver account number:");
        let mut account_number = String::new();
        io::stdin().read_line(&mut account_number).expect("Failed to read line");
        let account_number: u32 = account_number.trim().parse().expect("Please enter a number");
    
        println!("Enter receiver name:");
        let mut holder_name = String::new();
        io::stdin().read_line(&mut holder_name).expect("Failed to read line");
        let holder_name = holder_name.trim().to_string();
    
        BankAccount {
            account_number,
            holder_name,
            balance: 4000.0,
        }


    }
    println!("Depositer's balance: {}", account2.balance());
    println!("Receiver's Balance:{}", account1.balance());
}

    



    

trait Account{
    fn deposit(&mut self, money:f32);
    fn withdraw(&mut self, money:f32);
    fn balance(&self)->f32;
}
struct BankAccount{
    account_number: u32,
    holder_name:String,
    balance: f32,
}
impl Account for BankAccount{
    fn deposit(&mut self, money:f32){
        self.balance +=money; 
    }

    fn withdraw(&mut self, money:f32){
        self.balance -= money;
    }

    fn balance(&self)->f32{
        self.balance
    }
    
}
