fn main() {
    let name = "Ben";
    let age = 26;
    // A variable is immutable, so if we try to re-assign age = 12, it will throw an error. If you
    // want to change the variable value then instantiate it with `mut` keyword
    let mut weight = 70;
    println!("Hi, my name is {}, and I am {} years old. My weight is {} kg.", name, age, weight);

    weight = 68;
    println!("Hi, my name is {}, and I am {} years old. My weight is {} kg.", name, age, weight);
}
