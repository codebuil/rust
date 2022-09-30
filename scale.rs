

// This is the main function
fn main() {
// This is the main function
    
    // Print text to the console
        
   let mut _n:f64=0.00;
   let mut _nn:i64=0;
   for _nn in 0 .. 11{
          _n= _nn as f64;
          println!("\x1bc\x1b[42;30m    {}]\n",scall(_n));
    }
    
}


fn scall(a:f64)->f64{
    let mut aa:f64=a;
    if a>10.00{
        aa=10.00;
    }
    if a<0.00{
        aa=0.00;
    }
    aa=aa*10.00;
    return aa;
    
    
}
