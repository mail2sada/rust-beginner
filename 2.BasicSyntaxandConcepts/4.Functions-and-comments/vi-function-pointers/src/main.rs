//Demonsatraits passing function as agrument
fn main() {
    println!("Demo: Function pointers");
    functionPointerDemo(|x| println!("Valure received is {}",x));

}

fn functionPointerDemo(f:Fn(msg:&str)) {
    f("Hello! this demo function");
}
