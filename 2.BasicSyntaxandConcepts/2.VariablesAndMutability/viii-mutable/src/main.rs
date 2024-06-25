/// Demonstraits mutable variable how to create
/// Usage of keyword mut
fn main() {
    println!("Demo: Mutable variable");

    let mut x = 100; // mut key word will make variable x to change its value

    println!("Value of variable x {}", x);
    // last example we have see below line resuted in error
    x = 500;

    println!("Value of variable x {}, after mutation", x)
}
