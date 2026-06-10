fn main(){
    // #[derive(Debug)]
    let a_float : f64 = 23.90;
    println!("{}",a_float);

    let b_float = 56.90f64;
    println!("{}",b_float);

    let mut a_integer = 54i32;
    println!("{}",a_integer);
    a_integer = 67i32;
    println!("{}",a_integer);

    // a_integer = 89.90f64;
    // println!("{}",a_integer);

    let a = 89i32;
    let b = 77i32;
    println!("a + b = {}",a+b);

    let mut c = 88.90f64;
    let d = 76.43f64;

    println!("c + d = {:.2}",c + d);

    println!("c*d = {:.2}",c*d);

    c = 0f64;
    println!("d/c = {}",d/c);
    // let e = 67i32;
    // let f = 0i32;

    // println!("e/f = {}",e/f);
    // Rust follows IEEE-754 standard for flaoting-point numbers, so dividing an f64 by 0.0 produces special values like inf,-inf,or NaN instead of an error. Intger types such as i32 don't have representations for infinity, so dividing by zero is undefined and Rust raise a runtime panic to prevent invalid calculations.

    let my_tuple = (90i32, 90.98f64, 100i32, true, false, "Yash");

    println!("Number = {}",my_tuple.0);
    println!("Float = {}",my_tuple.1);
    println!("Number = {}",my_tuple.2);
    println!("Bool = {}",my_tuple.3);
    println!("Bool = {}",my_tuple.4);
    println!("Number = {}",my_tuple.5);

    println!("{:?}", my_tuple);
    let my_tuple1 = (
        "Yash",
        1.90f64,
        ("Candy",90i32, true),
        true,
        false
    );
    println!("inner1 = {}",my_tuple1.2.0);
    println!("inner2 = {}",my_tuple1.2.1);
    println!("inner3 = {}",my_tuple1.2.2);

    let mut courses = ("AI","ML","DL");
    courses.1 = "RL";
    println!("courses1 = {}",courses.1);

    let (course1,_,course3) = courses;
    println!("{}, {}",course1,course3);
    

}