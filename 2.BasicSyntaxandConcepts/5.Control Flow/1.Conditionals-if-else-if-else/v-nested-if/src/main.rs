/// Demonstrates nested if
fn main() {
    println!("Demo:Nested if");
    let number = 8;

    if number % 2 == 0 {
        if number % 4 == 0 {
            println!("The number is divisible by 4.");
        } else {
            println!("The number is even but not divisible by 4.");
        }
    } else {
        println!("The number is odd.");
    }
}
