// Variables hold primitive data or references to data
// Varibales are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // immutable variable name - cannot change
    let name = "Bhaskar";

    // mutable variable age - can update values
    let mut age = 23;
    age += 1;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 369;
    println!("ID is {}", ID);

    // Assigning multiple variables
    let (my_name, my_age) = ("Bhaskar", 23);
    println!("{} is {}", my_name, my_age);
}
