
static mut LL:String=String::new();

// This is the main function
fn main() {
// This is the main function
    
    // Print text to the console
        
   starts(); 
    println!("\x1bc\x1b[42;30m    {}]\n",ret0());
    
    
}
fn starts(){
   
   let mut  _l:String="*".to_string();
   unsafe{
    LL.reserve(20);
    LL=_l.repeat(17);
   }
   
}

fn ret0()->String{
    unsafe{
    return LL.clone();
    }
    
}
