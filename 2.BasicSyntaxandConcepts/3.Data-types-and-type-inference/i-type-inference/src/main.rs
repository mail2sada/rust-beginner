/// Demonsatates rust type inference
fn main() {
    let x = 42; // Rust infers x to be an i32
    println!("The value of x is: {}", x);
    
    let y = 3.14; // Rust infers y to be an f64
    println!("The value of y is: {}", y);
    
    let greeting = "Hello, Rust!"; // Rust infers greeting to be a string (&str)
    println!("{}", greeting);
}
