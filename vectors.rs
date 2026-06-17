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
}