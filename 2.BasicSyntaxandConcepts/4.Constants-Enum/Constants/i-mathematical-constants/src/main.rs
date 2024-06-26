/// Demonstrates declaration of constants with example of circumfrence and exponential
const PI: f64 = 3.14159;
const E: f64 = 2.71828;

fn main() {
    println!("Demo:Usage of constants");

    let radius = 5.0;
    let circumference = 2.0 * PI * radius;
    let exponential = E.powf(2.0);

    println!("Circumference: {}", circumference);
    println!("Exponential of E: {}", exponential);
}
