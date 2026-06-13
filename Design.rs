// Designing a CLI Contact book

struct Contact {
    name: String,
    phone: String,
    email: String,
}
impl Contact {
    // Creating methods for Contact
    //1. Creating new contact details
    fn new(name: String, phone: String, email: String) ->Self{
        Self {
            name,
            phone,
            email,
        }

    }
    // Display method to display contact details
    fn display(&self){
        println!("Name: {}",self.name);
        println!("Phone: {}",self.phone);
        println!("Email: {}",self.email);
        println!("---------------------");

    }
}
//2. Adding contact details
fn add_contacts(contacts: &mut Vec<Contact>){
    contacts.push(Contact::new(
        String:: from("Yash"),
        String::from("9829672190"),
        String::from("vyash2428@gmail.com"),

    ));

}
// 3. Showing contact details
fn show_contacts(contacts: &Vec<Contact>){
    for contact in contacts {
        contact.display();
    }
}

enum Transaction{
    Deposit(f64),
    Withdraw(f64),
    CheckBalance,
    Exit,
}

struct Account {
    balance: f64,
}
impl Account{
    fn new(balance: f64)-> Self{
        Self{balance}
    }

    fn process_transaction(&mut self, Transaction: Transaction){
        match Transaction{
            Transaction:: Deposit(amount) => {
                self.balance += amount;
                println!("${} deposited successfully.",amount);

            }
            Transaction:: Withdraw(amount) =>{
                if self.balance >= amount {
                    self.balance-=amount;
                    println!("${} withdrawn successfully.", amount);
                } else{
                    println!("Insuffecient Balance!");

                } 
            }
            Transaction:: CheckBalance =>{
                println!("Current Balance: ${}",self.balance);
            }
            Transaction:: Exit => {
                println!("Thank you for your transaction");
            }
        }
    }
}
enum FoodVariety{
    Veg(String),
    NonVeg(String),
    Beverage(String),
}

struct FoodItem{
    order: String,
}
impl FoodItem{
    fn new(order: String) -> Self{
        Self{order}
    }
    fn ordering_food(&self, Food_Variety: FoodVariety){
        match Food_Variety{
            FoodVariety:: Veg(vegfood) =>{
                println!("I would like to order {}",vegfood);
            }

            FoodVariety:: NonVeg(nonveg) =>{
                println!("I would like to order {}",nonveg);
            }

            FoodVariety:: Beverage(beverage) =>{
                println!("I would like to order {}",beverage);
            }
        }
    }
}

fn main(){
    let mut contacts =Vec::new();
    add_contacts(&mut contacts);
    show_contacts(&contacts);

    let mut account = Account:: new(10000.0);
    account.process_transaction(
        Transaction:: Deposit(2000.0)
    );
    account.process_transaction(
        Transaction::Withdraw(1500.0)
    );

    account.process_transaction(
        Transaction::CheckBalance
    );


    let food = FoodItem:: new(String::from("Customer Order"));

    food.ordering_food(
        FoodVariety::Veg(String:: from("Panner Butter Masala"))
    );
}