fn main() {
  // Arithmetic Operators
  
  // Arithmetic operators are used to do basic math:
  
  let add = 5 + 3;
  let sub = 10 - 4;
  let mul = 6 * 2;
  let div = 12 / 3;
  let rem = 10 % 3;

  println!("Add: {}", add);
  println!("Sub: {}", sub);
  println!("Mul: {}", mul);
  println!("Div: {}", div);
  println!("Rem: {}", rem);

  // Assignment Operators
  
  // Assignment operators are used to assign and update values:

  let mut x = 10;
  println!("Start: {}", x);

  x += 5;
  println!("After += 5: {}", x);

  x -= 2;
  println!("After -= 2: {}", x);

  x *= 2;
  println!("After *= 2: {}", x);

  x /= 3;
  println!("After /= 3: {}", x);

  x %= 4;
  println!("After %= 4: {}", x);

  // Comparison Operators
  
  // Comparison operators compare values and return true or false:

  let a = 5;
  let b = 10;

  println!("5 == 10: {}", a == b);
  println!("5 != 10: {}", a != b);
  println!("5 < 10: {}", a < b);
  println!("5 >= 10: {}", a >= b);

  //Logical Operators
  
  //Logical operators are used to work with boolean values:
  
  let logged_in = true;
  let is_admin = false;

  println!("Is regular user: {}", logged_in && !is_admin);
  println!("Has any access: {}", logged_in || is_admin);
  println!("Not logged in: {}", !logged_in);
}
