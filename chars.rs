// This is the main function
fn main() {

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",rchar(3));
}

fn rchar(_i:i8)->char{
    let l=['a','b','c','d'];
    let ii:usize=_i as usize;
    return l[ii];
}
