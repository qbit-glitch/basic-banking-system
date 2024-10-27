fn main() {
    let mut bank_account_1 = BankAccount{
        account_number: 8976345678914365,
        holder_name: "Umesh Kumar Singh".to_string(),
        balance: 1900.60, 
    };

    let mut bank_account_2 = BankAccount{
        account_number: 8769354287651034,
        holder_name: "Amrita Singh".to_string(),
        balance: 3400.78,
    };
    println!("Account-1 Balance: {}, Account-2 Balance: {} ", bank_account_1.balance, bank_account_2.balance);
    bank_account_1.deposit(500.0);
    bank_account_2.withdraw(500.0);
    println!("Account-1 Balance: {}, Account-2 Balance: {} ", bank_account_1.balance(), bank_account_2.balance());

}


trait Account{
    fn deposit(&mut self, deposit:f64);
    fn withdraw(&mut self, withdraw:f64);
    fn balance(&mut self) -> f64;
}

struct BankAccount{
    account_number : i64,
    holder_name: String,
    balance : f64,
}

impl Account for BankAccount{
    fn deposit(&mut self, deposit: f64){
        self.balance += deposit;
    }

    fn withdraw(&mut self, withdraw: f64){
        if withdraw <= self.balance
            {self.balance -= withdraw;}
        else
            {println!("Low Balance. Cannot withdraw {} amount.", withdraw);}
    }

    fn balance(&mut self) -> f64{
        self.balance
    }
}