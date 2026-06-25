// From and Into
use std:: convert::From;
#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) -> Self{
        Number {value: item}
    }
}
// From<T> for U -> Defines how to convert T into U
// Without From
struct Celsius{
    temperature: f64,
}
#[derive(Debug)]
struct Fahernheit{
    temperature: f64,
}

// impl Fahernheit{
//     fn from_celsius(c: Celsius)->Self {
//         Fahernheit{
//             temperature: c.temperature* 9.0/5.0 + 32.0,
//         }
//     }
// }

impl From<Celsius> for Fahernheit {
    fn from(c:Celsius) ->Self{
        Fahernheit {
            temperature: c.temperature*9.0/5.0 + 32.0
        }
    }
}
fn main(){
    let num  = Number:: from(30);
    println!("My number is {:?}",num);

    //let c = Celsius{temperature: 25.0};
    //let f = Fahernheit::from_celsius(c);
    //println!("Temperature in Fahernheit: {:?}",f);
    let c1 = Celsius {temperature: 25.0};
    let f = Fahernheit::from(c1);
    println!("{}", f.temperature);

    let c3 = Celsius {temperature: 25.0};
    let f1: Fahernheit = c3.into();

    println!("{}",f.temperature);

}