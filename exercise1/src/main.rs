#[allow(dead_code)]
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

fn take_ownership(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let account = Account::new(1, String::from("me"));

    // TODO: Write and call a function that will *take ownership* of the Account
    // value, print it, and return nothing
    take_ownership(&account);

    // Question: Will you be able to call the function twice with the 'account'
    // variable?
    take_ownership(&account);
}