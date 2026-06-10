// Different Primitives

fn main(){
    let logical: bool = true;
    let a_float: f64 = 1.78; // Regual annotation
    let a_integer = 5i32; // Suffix annotation
    let c_integer: i32 = 7; // Regualar annotation
    println!("{}",logical);
    println!("{}",a_float);
    println!("{}",a_integer);
    println!("{}",c_integer);

    // Default 
    let default_float = 3.0; //f64
    let default_integer = 7; // i32
    println!("{}",default_float);
    println!("{}",default_integer);
    let mut inferred_type = 12; // This code shows that the variable inferred_type is mutable 
    println!("{}",inferred_type);
    // By default the the variable defined in RUST are immutable
    inferred_type = 6789193;
    println!("{}",inferred_type);
    let mut mutable = 12;
    println!("{}",mutable);
    // mutable = true; // Here we are changing the type of mutable whci is not allowed in RUST. It will give an error
    // let mutable = true;
    // println!("{}",mutable);

    let my_array: [i32; 5] = [1,2,3,4,5]; // Array signature consists of Type T and length as [T; length];

    println!("{:?}",my_array);
    println!("{:#?}",my_array);

    for value in my_array{
        println!("{}",value);
    }

    let float_array: [f64; 6]  = [1.2,4.6,7.8,9.00,5.98,4.55];
    for val in  float_array{
        println!("{}",val);
    }

    let unsigned_int: u32 = 9;
    println!("{}",unsigned_int);
    let unsigned_float: f64 = 8.76;
    println!("{}",unsigned_float);
}