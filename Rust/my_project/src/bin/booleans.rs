fn main() {
    // You can store a boolean value in a variable using the bool type:
    let is_programming_fun: bool = true;
    let is_fish_tasty: bool = false;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);

    // Remember that Rust is smart enough to understand that true and false values are boolean values, meaning that you don't have to specify the bool keyword
    
    // Most of the time, there is no need to type true or false yourself. Instead, boolean values come from comparing values using operators like == or >:

    let age = 20;
    let can_vote = age >= 18;

    println!("Can vote? {}", can_vote);

    // Boolean values are often used in if statements to decide what code should run:
    
    let is_logged_in = true;

    if is_logged_in {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }
}
