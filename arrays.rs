use std::mem;
fn analyze_slice(slice: &[i32]){
    println!("First element of the slice: {}",slice[0]);
    println!("The slice has  {} elements",slice.len());

}
fn main(){
    // Fixed - size array

    let xs: [i32;5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0;500];
    println!("First element of the array is : {}",xs[0]);
    println!("Second element of the array is: {}",xs[1]);


    // len returns the count od elements in the array
    println!("Number of elements in array: {}",xs.len());

    // Arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // 'starting_index' is the first position in the slice.
    // 'ending_index' is the one more than the last position in the slice.
    println!("Borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice '&[]':
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); 

    let mut numbers = [90.93f64, 89.12f64, 19.23f64];
    numbers[2] = 76.22f64;

    println!("{:?}",numbers);

    for val in numbers {
        println!("{}",val);
    }
    
    for (i, value) in numbers.iter().enumerate(){
        println!("arr[{}] = {}",i, value);
    }

    let mut sum = 0f64;

    for val in numbers{
        sum = sum + val;
    }
    println!("Sum = {}",sum);
    // Slices

    let slice = &numbers[1..3];

    println!("{:?}", slice);

    let s: &[f64] = &numbers[1..3];
    println!("{:?}",s);

    let v = vec![1,2,3,4,5];
    let s1 = &v[1..4];
    println!("{:?}",s1);

    // Modfiying through Mutable slices
    let mut arr = [1,2,3,4];

    let slice = &mut arr[1..3];

    slice[0] = 20;
    slice[1] = 30;

    println!("{:?}", arr);

    let arr_1 = [1i32, 15i32, 20i32,90i32];

    let s2 = &arr_1[1..3];

    println!("{:?}",s2);

    let mut arr_2: [i32;5]  = [1,2,3,4,5];

    let mutslice = &mut arr_2[..];

    // mutslice[0] = 2;
    // mutslice[1] = 4;
    // mutslice[2] = 6;
    // mutslice[3] = 8;
    // mutslice[4] = 10;

    // Using for loop

    for num in mutslice.iter_mut(){
        *num *= 2;
    }


    println!("{:?}",mutslice);

}