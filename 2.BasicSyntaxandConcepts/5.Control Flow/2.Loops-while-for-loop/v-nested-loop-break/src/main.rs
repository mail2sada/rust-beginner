fn main() {
    println!("Example 5: Breaking Out of Inner and Outer Loops");
    
    'outer: for i in 1..=3 {
        println!("Outer loop iteration: {}", i);
        
        for j in 1..=2 {
            println!("Inner loop iteration: {}", j);
            
            if i == 2 && j == 2 {
                break 'outer; // Breaks out of both inner and outer loops
            }
        }
    }
}
