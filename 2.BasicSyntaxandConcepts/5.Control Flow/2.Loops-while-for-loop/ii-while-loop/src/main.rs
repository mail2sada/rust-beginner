///Demonstraing conditional loop with while loop.
fn main() {
    let mut number = 3;
    println!("Demo: Conditional Loop with `while`");
    
    while number >= 0 {
        println!("Number: {}", number);
        number -= 1;
    }
}
