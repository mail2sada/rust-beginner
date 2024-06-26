///Demonstraits working of the loop using loop key word.
///used to build infinite loop

fn main() {
    println!("Demo: Infinite Loop with `loop`");

    let mut count = 0;
    
    loop {
        count += 1;
        println!("Count: {}", count);
        
        if count == 3 {
            break; // Exit loop when count reaches 3
        }
    }
}
