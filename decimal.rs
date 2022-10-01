// This is the main function
fn main() {
    // This is the main function
    let mut _n:i128=0;
    let mut _m:i128=1;
        
    
     // Print text to the console 
     println!("\x1bc\x1b[42;30m\n");
     
     for _n in 0 .. 38{
         println!("d{}",_m);
         println!("{:x}",_m);
         _m=mults(_m,10);
     
     }
     
     } 
     fn mults(a:i128,b:i128)->i128{
          return a*b;
          
     }
