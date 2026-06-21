// Strings

//1 . Creating strings
fn main(){

let s1 = String::new();
let s2 = String::from("Yash");
println!("{}",s2);

// 2. push() a character
let mut s3 = String::from("Rust");
s3.push('!');
println!("{}",s3);
s3.push_str("Programming"); //3. push_str()
println!("{}",s3);

// 4. len() Returns number of bytes
let s4 = String:: from("Glück");
// Here we can see by using a German word which has a special character in it
// The output getting is 6. This shows that the len() return the number of bytes.
println!("{}",s4.len());

// 5. is_empty()
let s5 = String::new();
println!("{}",s5.is_empty());

// 6. contains()
let s6 = String::from("Rust Programming");
println!("{}",s6.contains(" "));
// 7. starts_with()
let s7 = String:: from("Rust");
println!("{}",s7.starts_with("Rr"));
// 8. ends_with()
let s8 = String:: from("Placements");
println!("{}",s8.ends_with("ts"));

// 9. replace()
let s9 = String::from("I love Rust");

let new_s = s9.replace("Rust","C++");

println!("{}",new_s);
// 10. to_uppercase

let s10 = String:: from("I love Rust Programming");
println!("{}",s10.to_uppercase());

// 11. to_uppercase

let s11 = String:: from("I AM A PROGRAMMER");
println!("{}",s11.to_lowercase());



}