// This is the main function
fn main() {
// This is the main function

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",rstr("hello world....\n".to_string(),3));
}

fn rstr(l:String,_i:i8)->String{
    let ii:usize=_i as usize;
    let iii:usize=l.len();
    let lll:String=l[ii..iii].to_string();
    return lll;
}
