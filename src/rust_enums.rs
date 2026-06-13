/*Enums
An enum (short for "enumeration") is a way to define a type that can be one of a few different values.

Each value in the enum is called a variant.

Enums are useful when you want to represent a value that can only be one of a set of options - like days of the week, directions, or results like success and error.
To create an enum, use the enum keyword and add a set of named values (variants) separated by commas:
*/

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum LoginStatus {
  Success(String),
  Error(String),
}

fn main(){
    let my_direction = Direction::Left;
    
    match my_direction {
    Direction::Up => println!("Going up"),
    Direction::Down => println!("Going down"),
    Direction::Left => println!("Going left"),
    Direction::Right => println!("Going right"),
  }

  let result1 = LoginStatus::Success(String::from("Welcome, John!"));
  let result2 = LoginStatus::Error(String::from("Incorrect password"));

  match result1 {
    LoginStatus::Success(message) => println!("Success: {}", message),
    LoginStatus::Error(message) => println!("Error: {}", message),
  }
}