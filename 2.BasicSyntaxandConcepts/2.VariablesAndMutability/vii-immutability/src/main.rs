///Demonstraits immutability
///Any variable declared is by default immutable
/// you can not change the value of it
/// We will see in the next example, how to make a variable mutable
/// viii-mutable
fn main() {
    println!("Demo:Immutability");
    let x = 100;

    println!("Value of x {}",x);

    // #12 will lead to compiliation error
    // please comment #12 for successful compilation
    x = 500; 
    //[Compilation error]^^^^^^^ cannot assign twice to immutable variable
    println!("Value of x after chainging {}",x)
}
