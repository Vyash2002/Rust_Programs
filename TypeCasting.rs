// Casting
//Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword.

fn main(){
    let decimal = 65.3451_f32;

    // let integer :u8 = decimal;
    // Here we cannot implicitly convert one type into another in Rust
    // We have to convert it explicitly

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {}->{}->{}",decimal, integer, character);
    

}