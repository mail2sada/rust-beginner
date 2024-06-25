///Demonstraits declaration of multiple variables
fn main() {
    
    println!("Demo: Multiple variable declaration");
    /// delcaring variables x, y, z in a single statement of homogeneous types
    let (x, y, z) = (10, 20, 30);
    println!("Value of x-{}, y-{}, z-{}", x, y, z);

    /// declaring variables a, b, c in a single statement of hetrogeneous types
    let (a, b, c) = (10, 3.142, "Hello");
    println!("Value of a-{}, b-{}, c-{}", a, b, c)
}
