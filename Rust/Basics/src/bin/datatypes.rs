fn main() {
    /*
    In Rust, the type of a variable is decided by the value you give it. 
    Rust looks at the value and automatically chooses the right type:
    */

    // If a variable is intentionally unused (e.g., for demonstration purposes),
    // prefix its name with an underscore (_) to suppress the "unused variable" warning.
    //
    // Example:
    // let _my_num = 5;
    // let _my_text = "Hello";

    let _my_num = 5;         // integer
    let _my_double = 5.99;   // float
    let _my_letter = 'D';    // character
    let _my_bool = true;     // boolean
    let _my_text = "Hello";  // string
    
    // However, it is possible to explicitly tell Rust what type a value should be:
    let _my_num: i32 = 5;          // integer
    let _my_double: f64 = 5.99;    // float
    let _my_letter: char = 'D';    // character
    let _my_bool: bool = true;     // boolean
    let _my_text: &str = "Hello";  // string

    // Combining data types
    let name = "John";
    let age = 28;
    let is_admin = false;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Admin: {}", is_admin);
}
