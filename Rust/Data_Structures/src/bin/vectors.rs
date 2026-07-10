/*
    For operations that require adding and removing array elements, you can use Vectors, which are resizable arrays.

    The size of a vector is dynamic, meaning it can grow and shrink as needed.

    You can use the vec! macro to create a vector.
*/

fn main() {
    // Appending to a vector

    let mut fruits = vec!["Apple", "Mango", "Orange"];

    fruits.push("Banana");

    println!("{:?}", fruits);

    // Removing last element from a vector

    fruits.pop();

    println!("{:?}", fruits);

    /*
        Rust vectors are designed to grow and shrink at the end, but you can also add or remove elements at the beginning or at a specified index.

        Use insert() to add an item at a specified index:
    */

    fruits.insert(2, "Grapes");

    println!("{:?}", fruits);

    // Use remove() to remove an element from a specified index

    fruits.remove(2);

    println!("{:?}", fruits);

    // Note: Adding or removing elements from the beginning is slower than at the end, because all the other elements have to shift positions.

    println!("There are {} fruits", fruits.len());

    // Looping through a vector

    for fruit in &fruits {
        println!("I like {}.", fruit);
    }

    /*
        Note: Use &fruits to borrow the vector instead of moving it.

        In Rust, borrowing means using a reference to a value instead of taking ownership of it. When you loop through a vector without &, the values are moved out, and you can no longer use the vector. But when you borrow the vector using &, you can still use it later in your program.
    */
}
