// main function
fn main() {
    // println!("Hello World!");
    println!("{} days", 31);
    // Normal print with {} which will be replaced by the arguements

    println!("{0}, this is {1}. {1}, this is {0}", "ALice","Bob");
    // Here {0}  and {1} show the posistional arguements basically we can say it is like indexing Alice->0 and Bob->1

    println!("{subject} {verb} {object}", object = "the lazy dog", subject = "the quick brown fox",verb = "jumps over");
    // Named Arguements here we can also use the positional arguements but it will create ambiguity

    // Different bases
    println!("Base 10:                {}", 69420);
    println!("Base 2 (binary):        {:b}", 69420); 
    println!("Base 8 (octal):         {:o}", 69420);
    println!("Base 16 (hexadecimal):  {:x}", 69420);
    // Right-justify text with specified width
    println!("{number:>5}", number=1); //00001
    // Padding extra zeroes to the left
    println!("{number:0>5}", number=1);
    // Appending extra zeroes to the right
    println!("{number:0<5}",number=1);
    // You can use named arguements in the format specifier by appending a '$'.
    println!("{number:0>width$}",number = 1, width = 5);
    // Rust checks that the aguements are correctly used or not
    // println!("My name is {0}, {1} {0}", "Bond"); // In this example we can see that one positional arguement is missing.
    // #[allow(dead_code)] // disable 'dead_code which warn against unused module
    // struct Structure(i32);
    // // fmt:Display;
    // println!("This struct '{}' won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");



}