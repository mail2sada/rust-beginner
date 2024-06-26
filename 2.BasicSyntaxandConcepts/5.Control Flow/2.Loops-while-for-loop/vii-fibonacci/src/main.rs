/// Demonstraiting fibonacci series

fn main() {
    println!("Demo: Fibonacci series...");
    let n = 10;
    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        println!("{}", curr);
        prev = curr;
        curr = next;
    }
    println!("{}", curr);
}
