fn main() {
    println!("Example 6: Returning Values from Loops");
    
    let mut sum = 0;
    let result = loop {
        sum += 1;
        
        if sum == 5 {
            break sum * 2; // Returns 10 and breaks the loop
        }
    };
    
    println!("Result from loop: {}", result);
}
