// This is the main function
fn main() {
// This is the main function
    
    // Print text to the console
    let _nn:i64;
    for _nn in 0 .. 16{
        
    
    println!("\x1bc\x1b[42;30m    {}]\n",power(2,_nn ));
    }
    
}

fn power(i1:i64,i2:i64)->i64{
    let mut i3:i64;
    i3=i1;
    let _n:i64;
    if i2==0 {
        return 1;
    }
    if i2==1{
        return i1;
    }
    for _n in  1 .. i2{
        i3=i3*i1;
    }
    
    return i3;
}
