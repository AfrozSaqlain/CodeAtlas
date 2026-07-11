/*
    A HashMap is a collection of key/value pairs.

    HashMaps are great when you want to store values and find them by a key.

    To use HashMap, you must import it from Rust's standard library:

                use std::collections::HashMap;
*/

use std::collections::HashMap;

fn main() {
    // You can create a new, empty HashMap and add items to it

    let mut capitalcities = HashMap::new();

    capitalcities.insert("India", "New Delhi");
    capitalcities.insert("Spain", "Madrid");
    capitalcities.insert("England", "London");
    
    println!("{:?}", capitalcities);

    // You can use the .get() method to access a value in a HashMap by its key

    if let Some(city) = capitalcities.get("India") {
        println!("The capital of India is {}", city);
    } else {
        println!("India not in the HashMap");
    }

    // If you insert a new value using a key that already exists, the old value is replaced with the new one

    capitalcities.insert("England", "Berlin");

    println!("{:?}", capitalcities);

    // To remove a key from a HashMap, use the .remove() method
    
    capitalcities.remove("England");

    println!("{:?}", capitalcities);

    // You can use a for loop to go through all key/value pairs
    
    for (country, city) in &capitalcities {
        println!("The capital city of {} is {}", country, city);
    }
}
