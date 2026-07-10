/*
    Now that we understand how functions work, it is important to learn how variables act inside and outside of functions.

    Scope refers to where a variable is allowed to be used.

    A variable only lives inside the block where it was created. A block is anything inside curly braces { }.
*/

fn main() {
    fn my_function() {
        let message = "Hello";

        println!("{}", message); // We can access the message variable here
    }

    my_function();

    // println!("{}", message); // This will throw an error as message variable is out of scope
    
    let score = 80;

    if score > 50 {
        let result = "Pass";

        println!("Result is {}", result);
    }

    // println!("Result is {}", result) // result is out of scope here

    
    // In Rust, you can declare a new variable with the same name in the same scope using let. This is called shadowing
    
    let x = 5;
    let x = 10;

    println!("x = {}", x);
    
    /*
        Here, the two x variables are in different scopes. The inner x only exists inside the block. Outside the block, the original value remains.
    */ 

    let y = 8;

    {
        let y = 12;

        println!("y inside the block {}", y)
    }

    println!("y outside the block {}", y)
}
