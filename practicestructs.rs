#[derive(Debug)]
struct Book{
    title: String,
    author: String,
    price: f64,
}
struct Car{
    Engine: String,
    Color: String,
    YOM: i32,
    Mileage: f64,
    TopSpeed: f64,
    SeatingSpace: i32,
    ishybrid: bool,
}
struct BankAccount{
    balance: f64,
    deposit: f64,
    withdraw: f64,

}
impl BankAccount{
    fn display_balance(&self){
        println!("Total Balance = {}",self.balance);
        println!("Deposited amount = {}",self.deposit);
        println!("Total balance after deposit of money = {}",self.balance+self.deposit);
        println!("Widthdraw amount = {}",self.withdraw);
        println!("Balance after withdraw = {}",self.balance - self.withdraw);
        println!("Total Balance = {}",self.balance);
    }
}
impl Car{
    fn start(&self){
        println!("Starting {} {} car!",self.Color,self.Engine);
    }
    fn display_info(&self){
        println!("Engine: {}",self.Engine);
        println!("Color: {}",self.Color);
        println!("Year of Manufacturing: {}",self.YOM);
        println!("Mileage: {}",self.Mileage);
        println!("TopSpeed: {}",self.TopSpeed);
        println!("Seating Space: {}",self.SeatingSpace);
        println!("Hybrid or not: {}",self.ishybrid);
    }
}
struct Point{
    x: u32,
    y: u32,
}
struct circle{
    radius: f64,
}
impl circle{
    fn area(&self) -> f64{
        self.radius*self.radius*3.14
    }
    fn circumference(&self) -> f64{
        self.radius*2.0*3.14
    }
}
fn main(){
    let b1  = Book{
        title: String:: from("Linear Algebra"),
        author: String:: from("Stinson"),
        price: 900.78,
    };

    println!("Book = {}",b1.title);
    println!("{:?}",b1);

    let mut b2 = Book{
        title: String::from("C Programming"),
        author: String:: from("Yashvanth Kanetkar"),
        price: 400.91,
    };
    b2.price = 450.89;
    println!("{:?}",b2);

    let c1 = Car{
        Engine: String::from("V8"),
        Color: String::from("Blue"),
        YOM: 2025,
        Mileage: 8.98,
        TopSpeed: 358.67,
        SeatingSpace: 4,
        ishybrid: false,
    };

    c1.start();
    c1.display_info();

    let point = Point {x:1,y:2};
    println!("X = {} and Y = {}",point.x,point.y);

    let cust1 = BankAccount{
        balance: 90000.67,
        deposit: 5487.90,
        withdraw: 43000.21,
    };

    cust1.display_balance();

    let circle1 = circle{
        radius: 9.87,
    };

    println!("Area of circle is = {:.2}",circle1.area());
    println!("Circumference of cicle = {:.2}",circle1.circumference());


}