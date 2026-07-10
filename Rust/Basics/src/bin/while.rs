fn main() {
    let mut count = 1;
    
    // The while loop checks the condition before each loop, so if the condition is false at the start, the loop will not run at all

    while count < 5 {
        println!("{}", count);
        count += 1;
    }

    // You can stop a `while` loop when you want by using `break`

    count = 1;

    while count < 10 {
        if count == 6 {
            break;
        }
        println!("Number: {}", count);
        count += 1;
    }

    // You can skip a value by using the `continue` statement

    count = 1;

    while count < 10 {
        if count == 7 {
            count += 1;
            continue;
        }

        println!("Number: {}", count);
        count += 1;
    }
}
