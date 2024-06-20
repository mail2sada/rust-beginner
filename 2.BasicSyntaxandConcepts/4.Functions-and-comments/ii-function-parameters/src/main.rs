// we will define a function that consumes a string argument and prints a hello message
// main function starting point of the execution
fn main() {
   let name = "Dilip Kumar";
   greet(name);
}
// function greet consumes a string and prints it 
fn greet(name:&str) {
    println!("Hello {}", name);
}
