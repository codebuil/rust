// This is the main function
fn main() {
// This is the main function
    let l:Vec<char>=['h','e','l','l','o',' ','w','o','r','l','d',' ','\n'].to_vec();
    // Print text to the console
    println!("\x1bc\x1b[42;30m    {}]",rstr(l));
}

fn rstr(ll:Vec<char>)->String{
    let lll:String= ll.iter().collect::<String>();
    return lll;
}
