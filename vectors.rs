// Exploring Different in build methods and macros in vectors;

fn main(){
    // 1 . Creating an empty vector
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(20);

    println!("{:?}",numbers);

    // 2 . Creating a vector using initial values
    let numbers1 = vec![1,2,3,4,5];
    println!("{:?}",numbers1);

    // 3. Creating a vector with same value repeated multiple times
    let zeroes = vec![0;5];
    println!("{:?}",zeroes);

    // 4. Crearing an empty vector with a specified capacity
    let mut numbers2 : Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}",numbers2.len()); // Here the length will be zero
    println!("Capacity: {}",numbers2.capacity());// The capacity is 10

    // 5. collect()
    // Convert an interator into a vector

    let numbers3:  Vec<i32> = (1..=5).collect();
    println!("{:?}",numbers3);

    let mut v1 : Vec<i32>  = Vec:: new();

    v1.push(100);
    v1.push(200);
    v1.push(300);
    v1.push(400);

    for i in v1{
        println!("{}",i);
    }

    let v2 = vec![32,90,12,22,12];

    for num in v2{
        println!("{}",num);
    }

    let twos = vec![2;5];

    for i in twos{
        println!("{}",i);
    }

    let mut numbers4:  Vec<i32> = Vec::with_capacity(12);
    numbers4.push(100);
    numbers4.push(200);
    numbers4.push(300);
    numbers4.push(400);

    println!("The length of the vector numbers4 is : {}",numbers4.len());
    println!("The capacity of the vector numbers4 is : {}",numbers4.capacity());

    let numbers5 : Vec<i32> = (3..=8).collect();

    println!("{:?}", numbers5);

    // 6. Vec :: from()
    // Create a vector from an array or slice

    let arr = [10,20,30];

    let numbers6 = Vec::from(arr);
    println!("{:?}", numbers6);

    // 7 to_vec()

    // Convert an array or slice into a vector

    let arr1 = [9,8,6,5,3];

    let number7 = arr1.to_vec();

    println!("{:?}",number7);

    // 8 repeat()
    // Create a vector by repeating elements of another vector or slice

    let v = vec![1,2];
    let repeated = v.repeat(3);

    println!("{:?}",repeated);

}