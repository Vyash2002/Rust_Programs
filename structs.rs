// Struct is a custom data type that lets us group related data together.
#[derive(Debug)]
struct Student{
    name: String,
    age: u32,
    cgpa: f64,
}

struct Point {
    x: f32,
    y: f32,
}

struct PointTuple(i32, i32);
struct Point3D{
    x: i32,
    y: i32,
    z: i32,
}

struct Rectangle {
    width: u32,
    height: u32,
}
struct Rectangle1 {
    width: u32,
    height: u32,
}

// Structs can have methods like impl. 
impl Rectangle1 {
    fn area1(&self) -> u32{
        self.width*self.height
    }
}
fn area(rect: &Rectangle) -> u32{
    rect.width*rect.height 
}
fn main(){
    let student1 = Student{
        name: String::from("Yash"),
        age: 22,
        cgpa: 6.08,
    };

    println!("{}",student1.name);
    println!("{:?}",student1);

    let mut student2 = Student{
        name: String::from("Raghav"),
        age:22,
        cgpa: 8.21,
    };
    student2.age = 23;
    println!("{:?}",student2);
    // Instantiate a 'Point'
    let point: Point = Point {x: 5.2, y: 0.4};
    let another_point: Point = Point {x: 10.3, y:0.2};
    // Access the fields of the point
    println!("point coordinates: ({},{})", point.x, point.y);
    // 'bottom_right.y' will be the same as 'another_point.y' because we used that field
    // from 'another_point'
    let bottom_right = Point {x: 10.3, ..another_point};
   
    
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);
     // Destructure the point using a 'let' binding
    let Point {x: left_edge, y: top_edge} = point;
    // Destructuring means breaking a complex data structure(tuple, struct, enum, et) into its individual parts and assigning them to variables

    let student3 = Student{
        name: String::from("Arjun"),
        age: 24,
        cgpa: 7.43,
    };

    let Student {name, age,cgpa} = student3;
    println!("name = {}, age = {},cgpa = {}",name,age,cgpa);

    // Ignoring Fields
    let p1 = Point3D {x:1, y:2, z:3};
    let Point3D {x, .. } = p1; // '..' means ignoring
    println!("{}",x);

    // Destructuring a tuple Struct

    let p2 = PointTuple(10,20);
    let PointTuple(x,y) = p2;
    println!("x = {}, y = {}",x,y);

    // Struct Update Syntax
    // Suppose you want to create a new struct based on an exsisting one.

    let s1 =  Student{
        name: String:: from("Yash"),
        age: 22,
        cgpa: 6.08,
    };

    let s2 = Student{
        age: 23,
        ..s1
    };

    println!("{:?}",s2);

    let r = Rectangle{
        width: 10,
        height: 5,
    };

    println!("Area = {}",area(&r));

    let rect1 = Rectangle1{
        width: 10,
        height: 5,
    };

    println!("{}",rect1.area1());





}