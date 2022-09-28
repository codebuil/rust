// This is the main function
fn main() {
// This is the main function

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",rxor(3));
}

fn rxor(_i:i8)->i8{
    let l=[65,66,67,68,69,70,71,72,73,74,75,76];
    let ii:usize=_i as usize;
    let _lll:i8=l[ii] as i8;
    let llll=_lll ^ 127;
    return llll;
}
