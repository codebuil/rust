// This is the main function
fn main() {

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",rchar(3));
}

fn rchar(_i:i32)->char{
    let l=['a','b','c','d'];
    return l[3];
}
