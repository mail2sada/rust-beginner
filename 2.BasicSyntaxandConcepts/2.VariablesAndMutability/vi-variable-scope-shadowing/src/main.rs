/// Demonstating variable scope and shadowing
fn main() {
    println!("Demo:Variable scope and shadowing");

    let x = 100;
    {
        // Variable x is shadowed only inside the scope, outside the scope it will retain its value
        let x = 500;
        println!("Value of variable x inside the scope{}", x);
        {
            // variable x is shadowed inside the inner scope again
            let x = x + 10;
            println!("Value of x inside inner scope {}", x)
        }
        println!("Value of variable x after executing inner scope inside the scope{}", x);
    }

    println!("Value of x {}", x);

    let s = 100;
    {
        let s = "We have shadowed s to a string";
        println!("Shadowed value of s{}", s)
    }
    println!("Value of s {}",s);
}
