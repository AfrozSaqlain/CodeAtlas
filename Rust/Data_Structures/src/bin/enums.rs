/*
    An enum (short for "enumeration") is a way to define a type that can be one of a few different values.

    Each value in the enum is called a variant.

    Enums are useful when you want to represent a value that can only be one of a set of options - like days of the week, directions, or results like success and error.
*/

// To create an enum, use the enum keyword and add a set of named values (variants) separated by commas

enum direction {
    Up,
    Down,
    Left,
    Right
}

// Enum variants can also hold data. This is useful when each variant needs to store extra information

enum LoginStatus {
    Success(String),
    Error(String),
}

fn main() {

    // To use the enum, create a variable and assign it one of the enum's variants (use `::` to access a variant)

    let my_direction = direction::Up;
    println!("We are going up!");

    // Enums work great with the match statement. You can run different code depending on which variant is used
    
    match my_direction {
        direction::Up => println!("Going up"),
        direction::Down => println!("Going down"),
        direction::Left => println!("Going left"),
        direction::Right => println!("Going right"),
    }

    let result1 = LoginStatus::Success(String::from("Welcome Ben!"));
    let result2 = LoginStatus::Error(String::from("Incorrect password!"));

    match result1 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        LoginStatus::Error(message) => println!("Error: {}", message),
    }
}
