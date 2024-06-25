/// Complier capability demonstration
/// Using a variable before initialization
/// usage of uninitialized variable
fn main() {
    println!("Demo: Usage of uninitialized varible");
    // declared variable x of type i32
    let x :i32 ;
    // this will result in an error,
    // for successful compilation please comment below line
    
    println!("Value of variable x {}", x);  

    // [Compiler error]^ `x` used here but it is possibly-uninitialized
    // rustc (rust compiler) prevents programmers to use uninitalized variables,
    // results in eliminating possible issue
    x =  100;
    println!("Value of variable x after initialixation {}", x)
}
