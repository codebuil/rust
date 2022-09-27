// This is the main function
fn main() {

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",mins(40,20));
}

fn mins(a:i32,b:i32)->i32{
    if a < b {
        return a;
    }else{
    return b;
    }
}