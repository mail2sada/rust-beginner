/// Demonstates rusts capability of shadowing variable(s)
fn main() {
    println!("Demo:Shadowing of variables");

    // declared variable x and assigned value 100
    let x = 100;

    println!("Value of variable x {}", x);

    // line below redeclares and x and assigns x+1, 
    //this is called shadowing

    let x = x + 1;

    println!("Value of x after shadowing {}", x);

    //shadowing variable with different type

    let x = "Welcome to rust trainig..";

    println!("X shadowed all together differnt type {}", x)
}
