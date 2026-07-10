/*
    Sometimes you want to use a value without taking ownership of it.

    Rust lets you do this using a reference - this is called borrowing.

    Since b is only borrowing the value, a still owns it.
*/

/*
    Borrowing helps you reuse values safely, without giving them away.

        - It lets you use values without taking ownership
        - It avoids cloning, which can be slow for large data
        - It makes your programs safer and faster
*/

fn main() {
    // A reference lets you look at a value without owning it. You create a reference using the `&` symbol
    
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);

    // If you want to change a value through a reference, you need to make the reference `mut`
    
    let mut name = String::from("Ben");
    let name_ref = &mut name;
    name_ref.push_str(" Tennyson");

    println!("Name is {}", name_ref);
    println!("Ref Name is {}", name);

    // Note: You can only have one mutable reference to a value at a time!
}
