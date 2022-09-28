// This is the main function
fn main() {
// This is the main function

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",rxor(3));
}

fn rxor(_i:i8)->i8{
    let l=String::from("hello world...\n");
    let ii:usize=_i as usize;
    let lll:Vec<char>=l.chars().collect();
    let l1=lll[ii] as i8;
    let llll:i8=l1 ^ 127;
    return llll;
}
