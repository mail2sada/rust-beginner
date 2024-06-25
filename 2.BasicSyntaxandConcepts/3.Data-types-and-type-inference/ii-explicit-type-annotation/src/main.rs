/// Demonstatites explict type annontation
fn main() {
    let x: i32 = 42; // Explicitly specifying type i32
    println!("The value of x is: {}", x);
    
    let y: f64 = 3.14; // Explicitly specifying type f64
    println!("The value of y is: {}", y);
    
    let greeting: &str = "Hello, Rust!"; // Explicitly specifying type &str
    println!("{}", greeting);
}
