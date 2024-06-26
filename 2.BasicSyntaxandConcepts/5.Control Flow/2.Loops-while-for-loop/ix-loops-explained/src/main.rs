fn main() {
    // Example 1: Infinite Loop with `loop`
    let mut count = 0;
    println!("Example 1: Infinite Loop with `loop`");
    loop {
        count += 1;
        println!("Count: {}", count);
        if count == 3 {
            break; // Exit loop when count reaches 3
        }
    }

    // Example 2: Conditional Loop with `while`
    let mut number = 3;
    println!("Example 2: Conditional Loop with `while`");
    while number > 0 {
        println!("Number: {}", number);
        number -= 1;
    }

    // Example 3: Iterating with `for` over a range
    println!("Example 3: Iterating with `for` over a range");
    for i in 1..=5 {
        println!("Iteration: {}", i);
    }

    // Example 4: Nested Loops
    println!("Example 4: Nested Loops");
    for i in 1..=3 {
        println!("Outer loop iteration: {}", i);
        for j in 1..=2 {
            println!("Inner loop iteration: {}", j);
        }
    }

    // Example 5: Breaking Out of Inner and Outer Loops
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

    // Example 6: Returning Values from Loops
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
