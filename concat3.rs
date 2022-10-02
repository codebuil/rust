fn main() {
    let mut _n:i64=0;
    let mut _s:String=String::from("->");
    let ss:String=String::from("<");
    let sss:String=String::from("-");
    let mut _ssss:String=String::new();
    println!("\x1b[42;30m\n");
    for _n in 0 .. 16 {
		_s=cat(sss.to_string(),_s.to_string());
		_ssss=cat(ss.to_string(),_s.to_string());
		println!("{}\n",_ssss);
	}
    
}

fn cat(s:String,ss:String)->String{
	let mut a:String=String::new();
	a.push_str(&s);
	a.push_str(&ss);
	return a;
}
