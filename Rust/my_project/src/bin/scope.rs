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
}
