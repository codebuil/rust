fn main() {
    let mut _n:i64=0;
    let mut _s:String=String::from("->");
    println!("\x1b[42;30m\n");
    for _n in 0 .. 16 {
		_s=cat(String::from("-"),_s);
		println!("{}\n",_s);
	}
    
}

fn cat(s:String,ss:String)->String{
	let mut a:String=String::new();
	a.push_str(&s);
	a.push_str(&ss);
	return a;
}
