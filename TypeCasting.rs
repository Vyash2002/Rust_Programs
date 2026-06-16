// Casting
//Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword.
#![allow(overflowing_literals)]
use std:: convert::TryFrom;
fn main(){
    let decimal = 65.3451_f32;

    // let integer :u8 = decimal;
    // Here we cannot implicitly convert one type into another in Rust
    // We have to convert it explicitly

    let integer = decimal as u8;
    let character = integer as char;
    // We cannt directly cast a decimal to character.
    // We need to convert it to integer and than to character
    println!("Casting: {}->{}->{}",decimal, integer, character);

    println!("1000 as a u16 is: {}",1000 as u16);
    println!("1000 as a u8 is: {}",1000 as u8);
    // As 1000 won't fit in u8 the Rust do a modulo operation on the value to squeeze the value into the range.
    // A u8 is 8 bits wide, So Rust effectively computes: 1000 %256
    // Because 256 is the number of distinct values au8 can represent.
    match u8::try_from(1000){
        Ok(value) => println!("{}",value),
        Err(_) => println!("Value doesn't fit in u8"),
    }
    // 1 + 256 = 255
    println!("-1 as a u8 is : {}",(-1i8) as u8);
    println!("128 as a i16 is: {}",128 as i16);
    println!("128 as a i8 is: {}",128 as i8);
    // In the example above we boundary case 128 value in 8-bit two's
    // complement representation
    println!("232 as a i8 is: {}", 232 as i8);
    // When we convert a u8-> i8 the first 0..127 remains same.
    // But 128-255 becomes negative.
    // We can calulate it by using the the formula result  = value- 256
    println!("1000 as a i8 is: {}",1000 as i8);
    println!("1000 as a i16 is: {}",1000 as i16);

    // Floating Numbers casting


    println!("300.0 as u8 is: {}",300.0_f32 as u8);
    // 300.0 as u8 when cast to int . If the floating value exceeds the 
    // upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is : {}", f32::NAN as u8);
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is : {}",300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }

}