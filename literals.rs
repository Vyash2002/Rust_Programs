//Literals and Operators

fn main(){
    //Integer Addition
    println!("1 + 2 = {}",1u32 + 2);
    println!("1 - 2 = {}",1i32 - 2);
    // println!("1 - 2 = {}",1u32 - 2);
    // The above code will give a buffer overflow problem as unsigned int has a range from 0 to 2^31 -1 and -1 is not in the range.
    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}",1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is  {}",1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // Basically {:04b} means print number in binary, using atleast 4 digits, and fill any missing digits on the left with 0's
    // Using underscores to improvde readability!
    println!("One million is written as {}",1_000_000u32);

    println!("3.90 + 67.90 = {:.2}", 3.90f64 + 67.90);
    println!("87.90 - 88.09 = {:.2}",87.90f64 -  88.09);
    println!("8*9 = {}",8i32*9);
    println!("9.90*67.90 = {:.2}",9.90f64*67.90);
    println!("77.90/54.98 = {:.3}",77.90f64/54.98);

    println!("78.90*76.90 + 89.90/9.01 - 87.90 = {}",78.90f64*76.90 + 89.90/9.01 - 87.90);




}
