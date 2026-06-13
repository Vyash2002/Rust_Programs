// Scope and Shadowing
// Variable bindings have a scope, and are constrained to lie in a block. 
// A block is a collection of statements enclosed by braces {}.

fn main(){
    let count1: i32 = 21;
    // The variable above count1 has a activation record till the function exists.

    {
        let count2: i32 = 34;
        // The variable count2 has activation record inside the this block only.
        // This is called scope.

        println!("The value of count2 = {}",count2);
    }

    // println!("The value of count2 = {}", count2);
    // The line above will give an error.
    println!("The value of count1 = {}", count1);


    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}",shadowed_binding);

        // This binding shadows the outer one
        let shadowed_binding = "abc";
        println!("shadowed in inner bloc: {}",shadowed_binding);

    }
    println!("outside inner block: {}",shadowed_binding);
    // This binding shadows the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}",shadowed_binding);

    // Declare First
    let a_binding;

    {
        let x = 2;
        // Initialize the binding
        a_binding = x*x;
    }
    println!("a binding: {}",a_binding);
    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}",another_binding);

    another_binding = 1;
    println!("another binding: {}",another_binding);

    // Freezing
    // When data is bound by the same name immutably, it also freezes.
    // Frozen data can't be modified until the immutable binding goes out of scope:

    let mut _mutable_integer = 8.90f64;
    {
        // Shadowing by immutable '_mutable_integer'
        let _mutable_integer = _mutable_integer;

        // Error! '_mutable_integer' is frozen in this scope
        _mutable_integer = 50.09;
        // _mutable_integer goes out of scope

    }
    // Ok! '_mutable_integer' is not frozen in this scope
    _mutable_integer = 3.1;


}