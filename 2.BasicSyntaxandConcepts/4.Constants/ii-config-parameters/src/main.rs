const MAX_POINTS: u32 = 100_000;
const DEFAULT_TIMEOUT: u64 = 30;

// we will cover functions in detail in further sections
fn connect_to_server() {
    println!("Connecting to server with timeout: {} seconds", DEFAULT_TIMEOUT);
    // Logic to connect to server with the specified timeout
}

fn main() {
    println!("Demo:Usage of constants");

    println!("Maximum points allowed: {}", MAX_POINTS);
    connect_to_server();
}
