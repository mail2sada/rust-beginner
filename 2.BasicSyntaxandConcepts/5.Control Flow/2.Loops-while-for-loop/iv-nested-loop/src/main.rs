/// Demonstraiting nested loop
fn main() {
    println!("Example 4: Nested Loops");
    
    for i in 1..=3 {
        println!("Outer loop iteration: {}", i);
        
        for j in 1..=2 {
            println!("Inner loop iteration: {}", j);
        }
    }
}
