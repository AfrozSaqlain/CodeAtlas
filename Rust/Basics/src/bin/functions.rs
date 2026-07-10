/* 
    To create a function, use the fn keyword, followed by the function name and a set of parentheses () and curly braces {}
    Everything has to be done in `main` function only
*/

fn main() {
    fn say_hello() {
        println!("Hello World!!!");
    }

    say_hello();

    // You can send information into a function using parameters. Parameters are written inside the parentheses ()

    fn greet(name: &str) {
        println!("Hello {}", name);
    }

    greet("Ben");

    /*
        A function can also return a value.

        Use the  -> symbol in the function header to show what type of value will be returned.

        Inside the function, use the return keyword to send the value back
        
        In Rust, you can omit the return keyword. Just write the value on the last line of the function, without a semicolon:
    */

    fn add(num1: i32, num2: i32) -> i32 {
        return num1 + num2;
    }


    fn add2(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    println!("Sum: {}", add(12, 24));
    println!("Sum: {}", add2(12, 24));
}
