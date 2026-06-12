enum TrafficLight {
    Red,
    Yellow,
    Green,
}
enum Season {
    Summer,
    Winter,
    Rainy,
    Spring,
}
enum Shape{
    Circle(f64),
    Rectangle(f64, f64),
}
impl Shape{
    fn area(&self) -> f64{
        match self{
            Shape::Circle(radius) => {
               3.14*radius*radius
            }
            Shape:: Rectangle(width,height) => {
               width*height
            }
        }
    }
}
impl Season{
    fn temperature_hint(&self){
        match self{
            Season::Summer => println!("It's too hot outside"),
            Season:: Winter =>println!("It's cold outside"),
            Season:: Rainy => println!("It's Rainy outside"),
            Season :: Spring => println!("The weather is awesome"),
        }
    }
}
enum Employee{
    Manager(String),
    Developer(String),
    Tester(String),
}
impl Employee{
    fn display_info(&self){
        match self{
            Employee:: Manager(name) => println!("Oh! {} is the manager of SDE team.",name),
            Employee:: Developer(name) => println!("Wow! {} is a Rust Developer",name),
            Employee:: Tester(name) => println!("Hey! {}, which test cases are you working on?",name),

        }
    }
}
enum Payment{
    Cash(f64),
    Card(String),
    UPI(String),
}

impl Payment{
    fn process_payment(&self){
        match self{
            Payment:: Cash(amount) =>println!("Hello sir, you have deposited {}",amount),
            Payment:: Card(card) => println!("Hi sir, Your payment for this package with card deatils {} has been successfully completed",card),
            Payment:: UPI(upi) => println!("Please, Enter your upi ID = {}",upi),
        }
    }
}
fn main(){
    let tf = TrafficLight::Red;

    match tf {
        TrafficLight::Red => println!("Stop it's Red Light"),
        TrafficLight::Yellow => println!("Be Ready!"),
        TrafficLight:: Green => println!("Go!"),
    }

    let current_season = Season::Winter;
    current_season.temperature_hint();

    let area1 = Shape::Circle(5.98);
    let area2 = Shape::Rectangle(9.87,7.54);

   println!("The area of cicle = {}",area1.area());
   println!("The area of rectangle = {}",area2.area());

   let e1 = Employee:: Manager(String::from("Yash"));
   e1.display_info();

   let e2 = Employee:: Developer(String :: from("Raghav"));
   e2.display_info();

   let p1 = Payment:: Cash(89314.87);
   p1.process_payment();

   let p2 = Payment:: Card(String:: from("8970-4567-9909-2234"));
   p2.process_payment();

   let p3 = Payment:: UPI(String:: from("yash@pil"));
   p3.process_payment();


}