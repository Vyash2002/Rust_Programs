// Debug-> Debug generally refers to the Debug Trait, which allows a value to be printed in a developer-friendly format for debugging purposes.

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
}
fn main(){
    let s = Student {
        name: String::from("Yash"),
        age:22,
    };
    println!("{:?}",s);
    println!("{:#?}",s);

}
// Output : Student {name: "Yash", age: 22} this out shows that it is the debug representation of the Student struct
// Here: Student-> the name of the struct(similar to a class name)
// name: "Yash"-> the value stored in the name field
// age: 22 -> the vlaue stired in the age field
// If we don't use the debug it will give an error it will not able to understand the format of the struct

