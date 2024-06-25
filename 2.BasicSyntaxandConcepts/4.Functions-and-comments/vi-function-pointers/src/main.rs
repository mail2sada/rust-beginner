//Demonsatraits passing function as agrument
fn main() {
    println!("Demo: Function pointers");
    functionPointerDemo(|x| println!("Valure received is {}",x));

}

fn functionPointerDemo<F>(f:&dyn Fn) 
where F: Fn(&str)
{
    f("Hello! this demo function");
}
