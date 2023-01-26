use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  
  let command = "hello";

  //let command = args[1].clone(); this is commented to avoid passing arguments every single time 
  
  
  let name = "Bhaskar";
  let status = "69%";

  // println!("Command: {}", command);

  if command == "hello" {
    println!("Hi {}, how are you?", name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    println!("That is not a valid command");
  }
}