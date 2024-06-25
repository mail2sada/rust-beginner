///Deminstraits mutable and immutable shadowing
fn main() {
    println!("Demo: Mutable and immutable shadowing...");

    let x = 100; // x is immutable

    println!("Value of x is {}", x);

    let mut x = x; // here the shadowed x is mutable

    println!("Value of mutable x {}", x);

    x = 500;

    println!("Value x {}, after the mutation ", x);

    // lets shadow x to immutable again 

    let x = x;
    println!("Value of immutable x {}", x);

    // below line will result in compulation errors

    x =  1000; // uncomment this line to compile and understand code

    println!("[]Value of immutable x {}", x);

}
