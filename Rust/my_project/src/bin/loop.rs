fn main() {
    // Below function will run forever. Hence I am commenting it out for the time being
    
    /*
    loop {
        println!("This will repeat forever!");
    }
    */

    // To stop a loop, use the `break` keyword:
    
    let mut count = 1;

    loop {
        println!("Hello World!");

        if count == 4 {
            break;
        }

        count += 1;
    }

    /*
        You can also return a value from a loop using break with a value.
    
        This lets you save the result of the loop into a variable

        Note: When you save the result of a loop into a variable, you must put a semicolon (;) at the end.
    */
    
    count = 1;
        
    let result = loop {
        println!("Hello World");

        if count == 4 {
            break count;
        }

        count += 1
    };

    println!("The loop stopped at {}", result);
}
