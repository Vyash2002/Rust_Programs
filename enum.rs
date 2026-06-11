// What is Enum
// An enum (enumeration) is a type that can have several possible variants.

enum Direction {
    North,
    South,
    East, 
    West,
}
impl Direction {
    fn display(&self){
        match self{
            Direction:: North => println!("North"),
            Direction:: South => println!("South"),
            Direction:: East => println!("East"),
            Direction:: West => println!("West"),

        }
    }
}
// A variable of type Direction can only be
// Direction:: North
// Direction:: South
// Direction:: East
// Direction:: West

// An enum represents one choice out of many possibilities

// Let say we take an example of Vechile
// struct Vechile{ is_car: bool, is_bike: bool, is_bus: bool}
// It doesn't make any sense
// Using enum will be preferable

enum Vechile{
    Car,
    Bike,
    Bus,
}
// Example 1: Basic Enum
#[derive(Debug)]
enum Day{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
// Example : Enum with data
enum IpAddress {
    V4(String),
    V6(String),
}
// Example : Different Variants, Different Data
#[derive(Debug)]
enum Message{
    Quit,
    Move(i32,i32),
    Write(String),
}

fn main(){
    let today = Day::Monday;
    println!("Today is {:?}",today);
    // Here we cannot directly write println!("Today is {}",today)
    // It will give an error because Rust knows the how to print built-in types
    // because they already implemented Display
    // But for custom enum Rust dosen't know what text we want to show, we must use either.
    // Use Debug ({:?} with #[derive(Debug)])
    // Implement Display 

    let dir = Direction::East;

    match dir{
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction:: East => println!("Going East"),
        Direction:: West => println!("Going West"),

    }
    let dir1 = Direction:: West;
    dir1.display();

    let home = IpAddress:: V4(String:: from("192.168.1.1"));
    let office = IpAddress:: V6 (String:: from("2001:db8::1"));
    // Here in all example we have seen we are using  :: symbol . it is called a path separator and serves a role similar to C++'s scope
    // resolution operator. It is used to access items associated with modules, enums, structs, traits, and crates
    // such as Day:: Monday, String:: from(), or std:: fmt. The '.' operator
    // on the other hand, is used to access fileds and methods of a specific instance.
    // let m1 = Message::Quit;
    // let m2 = Message::Move(10,20);
    // let m3 = Message::Write(String::from("Hello"));

    let msg = Message:: Move(10,20);
    match msg {
        Message:: Quit => println!("Quit"),
        Message:: Move(x,y) => {
            println!("Move to ({}, {})",x,y);
        }
        Message::Write(text) =>{
            println!("{}",text);
        }
    }



}