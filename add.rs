// This is the main function
fn main() {

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",adds(20,20));
}

fn adds(a:i32,b:i32)->i32{
    return a+b;
}