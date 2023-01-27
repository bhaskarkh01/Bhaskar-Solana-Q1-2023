pub fn run() {
    // Just prints to console
    println!("Hello people from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Rahul", "India");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Rahul", "India", "Code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Rahul",
        activity = "cricket"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Oct: {:o}", 69, 69, 69);

    // Placeholder for debug traits
    println!("{:?}", [1, 4, 3]);

    // Basic math
    println!(" 3 + 6 = {}", 3 + 6);
}
