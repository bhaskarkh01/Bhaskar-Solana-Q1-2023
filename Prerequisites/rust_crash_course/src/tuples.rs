// Tuples group together values of different data types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Bhaskar", "K H", 23);
  
    println!("{} is from {} and is {}", person.0, person.1, person.2);
  }