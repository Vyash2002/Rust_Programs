fn calculate(a: i32, b:i32) -> (i32,i32){
    (a+b,a*b)
}
fn main(){
    // Creating a tuple
    let person = ("Yash",22,true);

    println!("{:?}", person);

    // Accessing Tuple elements
    println!("Name: {}", person.0);
    println!("Age: {}",person.1);
    println!("Student: {}",person.2);

    // Tuple deconstructing
    let(name, age, student) = person;
    println!("{} {} {}", name, age, student);

    // Tuple with Different Data Types
    let data = (100, 3.14,'A',false);

    println!("Integer: {}",data.0);
    println!("Float: {}",data.1);
    println!("Char : {}",data.2);
    println!("Bool: {}",data.3);

    // Returning Multiple Vlaues from a Function
    let (sum, product) = calculate(10,20);

    println!("Sum = {}",sum);
    println!("Product = {}",product);

    // Nested Tuples

    let student = (
        "Yash",
        22, 
        ("AI","Machine Learning")
    );

    println!("Name: {}",student.0);
    println!("Age: {}",student.1);
    println!("Course1: {}",student.2.0);
    println!("Course2: {}",student.2.1);

    // Ignoring Tuple Values
    let(name1,_,status) = person;
    println!("{} {}",name, status);

    let mut person = ("Yash",22);

    person.1 = 23;
    println!("{:?}",person);

    // Single element tuple

    let x = (5,);
    println!("{:?}",x);
    



}