fn main() {
    // The last value is excluded

    for i in 1..6 {
        println!("Number: {}", i);
    }

    // If you want to include the last number, use ..= (two dots and an equals sign)

    for i in 1..=6 {
        println!("Number: {}", i);
    }

    // Just like other loops, you can use `break` to stop the loop and `continue` to skip a value
    
    for i in 1..=10 {
        if i == 5 {
            continue;
        }

        if i == 9 {
            break;
        }

        println!("Number: {}", i);
    }
}
