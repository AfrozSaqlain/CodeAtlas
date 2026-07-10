fn main() {
    // In below example `a` owns the string, but then we move it to `b`
    let a = String::from("Hello");

    let b = a;

    // println!("a = {}", a) // Error: a no longer owns the value
    
    println!("b = {}", b);
    
    /*
    When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid.

    But simple types like numbers, characters and booleans are copied, not moved.

    This means you can still use the original variable after assigning it to another
    */
    
    // Here, `a` is copied into `b`, not moved, so you can still use `b`.

    let a = 5;
    let b = a;

    println!("a = {}", a);
    println!("b = {}", b);

    /*
        For other types, like String, if you really want to keep the original value and also assign it to another variable,
        you can use the .clone() method, which makes a copy of the data
    */

    let a = String::from("Hello");
    let b = a.clone();

    println!("a = {}", a);
    println!("b = {}", b);

    // However, if you don't need to own the value twice, using a reference (&) is usually better than cloning, which you will learn more about in the borrowing.rs.
}
