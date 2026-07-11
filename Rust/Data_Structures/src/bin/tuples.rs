/*
    A tuple is a group of values of different types, stored in a single variable.

    Tuples are useful when you want to return or work with multiple values together.
*/

fn main() {
    // This tuple contains a &str, an i32, and a bool.
    
    // When we create a tuple, we normally assign values to it. This is called "packing" a tuple:
    
    let person = ("Ben", "15", true);

    println!("Name = {}, Age = {}, Is active = {}", person.0, person.1, person.2);

    //  In Rust, we are also allowed to extract the values back into variables. This is called "unpacking".

    let (name, age, active) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active);

    // Tuples are often used to return multiple values from a function

    fn get_user() -> (String, i32) {
        return (String::from("Ben"), 15)
    }

    let user = get_user();

    println!("Name = {}, Age = {}", user.0, user.1)
}
