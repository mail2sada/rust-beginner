fn main() {
    
    println!("Demo:function returning value");
    let (a, b) =  (200, 300);
    let c = add(a,b);
    println!("Value of c returned from add function is {}", c);
}

fn add(a:i32, b:i32)->i32 {
    return a+b;
}


