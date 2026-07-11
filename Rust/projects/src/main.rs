fn main() {
    fn fibonacci(num: i32) -> Vec<i32> {
        let mut result = vec![1, 1];
        if num == 1 {
            return vec![1];
        } else if num == 2 {
            return result;
        } else {
            for _i in 3..=num {
                result.push(result[result.len() - 2] + result[result.len() - 1]);
            }
            return result;
        }
    }
    let num: i32 = 3;
    println!("Fibonacci squence till {} is {:?}", num, fibonacci(num));
}
