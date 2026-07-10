fn main() {
    /* Constant variables are used to store values that never change.

    Unlike regular variables, constants must be defined with a type (e.g. i32 or char).

    Another thing about constants, is that it is considered good practice to declare them with uppercase.

    It is not required, but useful for code readability and common for Rust programmers:
    */
    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR: i32 = 60;

    println!("BIRTHYEAR = {}, MINUTES_PER_HOUR = {}", BIRTHYEAR, MINUTES_PER_HOUR)
}
