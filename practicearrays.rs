// Practicing arrays and Slices
use std::mem;
fn main(){
    let mut arr: [i32;7] = [90,91,89,32,12,11,10];

    println!("First element = {}",arr[0]);
    println!("Second element = {}",arr[1]);

    for val in arr{
        println!("{}",val);
    }

    println!("{:?}", arr);
    println!("Length of the array = {}",arr.len());

    for (mut num) in arr{
        num = num*2;
        println!("{}",num);
    }
    // Here we can analyze that in the first for loop above num is acting as a local variable
    // it will effect the current values in the for loop . If we try to print the array values again it will print the previous values that is the original values.
    for val in arr{
        println!("{}",val);
    }

    // We can use the '*' before num it will act like a pointer to that array *num : &mut arr [i] where i show the index.
    // Also iter_mut() will work
    // for num in arr.iter_mut() {
    //     *num*=2;

    // }
    // // Here above we can see that the 
    // for val in arr{
    //     println!("{}",val);
    // }

    let slice  = &mut arr[1..5];
    println!("The first element of the slice is: {}",slice[0]);

    for val in slice.iter_mut(){
        *val*=2;
    }
    for num in slice{
        println!("{}",num);
    }

    for(i, value) in arr.iter().enumerate(){
        println!("arr[{}] = {}",i,value);
    }

}