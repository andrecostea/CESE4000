use std::mem::size_of;

enum BankAccount {
    Account {
        account_number: String,
        owner: String,
        balance: f64,
    },
}

// instead, struct provides a more ergonomic way:
struct BankAccountStruct {
    account_number: String,
    owner: String,
    balance: f64,
}

fn main() {
   use BankAccount::Account;

    //initiate an enum variant
    let acc = Account {
        account_number: "123".into(),
        owner: "Alice".into(),
        balance: 1000.0,
    };

    //initiate a struct
    let sacc = BankAccountStruct{
        account_number: "123".into(),
        owner: "Alice".into(),
        balance: 1000.0,
    };


    match acc {
        Account { account_number, owner: o, balance: b } => {
            println!("Account number: {}", account_number);
        }
    }

    // instead of the more ergonomic way that struct provides:
    println!("Account number: {}", sacc.account_number);

    println!("Struct size: {}", size_of::<BankAccountStruct>());
    println!("Enum size:   {}", size_of::<BankAccount>());
}
