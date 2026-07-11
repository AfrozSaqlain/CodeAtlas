/*
    A struct (short for "structure") is a custom data structure that lets you group related values together.

    You can think of a struct like a mini-database for one thing, like a person with a name and age.
*/

fn main() {
    // You define a struct using the `struct` keyword and place the fields (variables) inside

    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }

    /*
        Once you have a struct, you can create an object of it.

        Then, you can access the fields of the struct using dot syntax (.)
    */

    let user = Person {
        name: String::from("Ben"),
        age: 15,
        can_vote: false,
    };

    println!("Name = {}", user.name);
    println!("Age = {}", user.age);
    println!("Can vote = {}", user.can_vote);

    // To change a value inside a struct, you must make the struct object mutable by using mut


    let mut user2 = Person {
        name: String::from("Ben"),
        age: 15,
        can_vote: false,
    };

    user2.age = 26;
    user2.can_vote = true;

    println!("Name = {}", user2.name);
    println!("Age = {}", user2.age);
    println!("Can vote = {}", user2.can_vote);
}
