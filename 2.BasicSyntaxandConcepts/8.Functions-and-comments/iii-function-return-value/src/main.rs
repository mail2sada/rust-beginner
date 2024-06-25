// We will implement function to add 2 numbers 
// add function will consume 2 i32 and return i32
//main function starting point of the execution
fn main() {
    println!("Demo: Functions returning a value");
    let a = 100;
    let b = 200;
    let c = add(a,b);
    println!("Value of c returned from add function is {}", c)
}

fn add(a:i32, b:i32)->i32 {
    return a+b;
}
