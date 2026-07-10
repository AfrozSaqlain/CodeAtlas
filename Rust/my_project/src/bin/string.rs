fn main() {
    let greeting: &str = "Hello";

    println!("{}", greeting);

    /*
        There are two main types of strings in Rust:

            - &str - is called "string slices", and is used for fixed text like "Hello"
            - String - used when you need a string that can change
    */
    
    // You can create a `String` from a string literal using the `to_string()` method or the `String::from()` function
    
    let text1 = "Hello World".to_string();
    let text2 = String::from("Hello World");

    println!("{}", text1);
    println!("{}", text2);
    
    /*
        Strings are mutable, so you can change them if they are declared with mut.

        Use push_str() to add text to a string
    */

    let mut greeting = String::from("Hello");
    greeting.push_str(" World");
    println!("{}", greeting);

    // Use push() to add one character
    
    let mut word = String::from("Hi");
    word.push('!');
    println!("{}", word);

    // You can combine strings using the format! macro
    
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");

    let result = format!("{} {} {}", s1, s2, s3);

    println!("{}", result);

    // You can also use the + operator to combine strings, but it can get messy with many value

    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = s1 + " " + &s2 + " " + &s3;
    
    println!("{}", result);

    // Note: You can only add a &str to a String with +. That is why &s2 and &s3 (a string slice) is used here

    let name = String::from("Ben");
    println!("Length of name {} is {}", name, name.len());
}
