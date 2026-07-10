/*
    An array in Rust is a fixed-size list of values, all of the same type.

    You cannot grow or shrink an array after it's created.

    To access an array element, refer to its index number.

    Array indexes start with 0: [0] is the first element, [1] is the second element, etc.
*/ 

fn main() {
    let fruits = ["Apple", "Mango", "Banana", "Orange"];
    println!("Last fruit in the basket is {}", fruits[fruits.len() -1]);

    /* 
        To change the value of a specified element, refer to the index number and assign a new value.

        Remember to make the array mutable (using the mut keyword)
    */

    let mut numbers = [1, 2, 3, 4, 5, 6, 7];
    numbers[numbers.len() - 1] = 9;

    println!("Last number is {}", numbers[numbers.len() - 1]);

    // You can loop through the array elements with the for loop

    for fruit in fruits {
        println!("Fruit = {}", fruit);
    }

    // Note: When printing the whole array, you must use `{:?}` inside println!

    println!("{:?}", fruits);
}
