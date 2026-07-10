fn main() {
    let age = 20;

    if age > 18 {
        println!("Can ride the roller coster!");
    } else {
        println!("Cannot ride roller coster!");
    }

    let money: f64 = 2000.0;
    const TICKET: f64 = 3000.0;

    let ride = if money > TICKET { "Can afford ticket" } else { "Cannot afford ticket" };

    print!("{}\n", ride);
}
