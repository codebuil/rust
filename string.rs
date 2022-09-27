// This is the main function
fn main() {

    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",rstring());
}

fn rstring()->String{
    
    return String::from("hello world");
}