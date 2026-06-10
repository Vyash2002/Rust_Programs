// Display -> unlike Debug it is basically user friendly . It is used to display the output for the users.

use std:: fmt;

struct Student {
    name: String,
    age: u32,
}

impl fmt:: Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} is {} years old",self.name, self.age)
    }
}
fn main(){
    let s = Student {
        name: String::from("Yash"),
        age:22,
    };
    println!("{}",s);
}
//Display is a Rust trait used for user-facing output and is invoked using {} in formatting macros like println!. Unlike Debug, which is intended for developers and can be derived automatically, Display must usually be implemented manually to define how a custom type should be presented to users.