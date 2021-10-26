#![allow(unused)]
use std::process;
use std::fmt;

enum LFAEDS{
	Num(i32),
	Add{l:Box<LFAEDS>, r:Box<LFAEDS>},
	Sub{l:Box<LFAEDS>, r:Box<LFAEDS>},
	Error,
}

impl fmt::Display for LFAEDS {
	fn 
	fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		match &*self{
			LFAEDS::Num(val) => write!(f, "(num {})", val),
			LFAEDS::Add{l, r} => write!(f, "(add {} {})", l, r),
			LFAEDS::Sub{l, r} => write!(f, "(sub {} {})", l, r),
			LFAEDS::Error => write!(f, "error"),
		}
	}
}

fn
parser(exp:&str) -> LFAEDS
{
	//println!("+_{}_+", exp);
	let sexp = split_exp(exp);
	if sexp.len() == 1 && sexp[0].parse::<i32>().is_ok() {
		return LFAEDS::Num(sexp[0].parse::<i32>().unwrap());
	}
	else if sexp[0].chars().nth(0).unwrap() == '+' {
		return LFAEDS::Add{l:Box::new(parser(&sexp[1])), r:Box::new(parser(&sexp[2]))};
	}
	else if sexp[0].chars().nth(0).unwrap() == '-' {
		return LFAEDS::Sub{l:Box::new(parser(&sexp[1])), r:Box::new(parser(&sexp[2]))};
	}
	return LFAEDS::Error;
}

fn
split_exp(exp:&str) -> Vec<String>
{
	if (exp.chars().nth(0).unwrap() == '{' && exp.chars().last().unwrap() != '}')
		|| (exp.chars().nth(0).unwrap() != '{' && exp.chars().last().unwrap() == '}') {
		//println!("input error");
		//process::exit(1);
		println!("__{}__", exp);
		panic!("input error");
	}
	let mut start_ind = 0;
	let mut end_ind = exp.len();
	if exp.chars().nth(0).unwrap() == '{' {
		start_ind = 1;
		end_ind -= 1;
	}
	let example_code = &exp[start_ind..end_ind];
	let mut brkt_cnt = 0;
	let mut buffer:String = "".to_string();
	let mut sexp:Vec<String> = Vec::new();
	for c in example_code.chars() {
		//println!("{}", c);
		/*
		if c == '{' {
			buffer.push_str(&c.to_string());
			continue;
		}
		else 
		*/
		if c == ' ' && brkt_cnt == 0 {
			if !buffer.is_empty() {
				sexp.push(buffer);
				buffer = "".to_string();
			}
		}
		/*
		else if c == '{' && brkt_cnt == 0 {
			brkt_cnt += 1;
			buffer = c.to_string();
		}
		else if c == '{' {
			brkt_cnt += 1;
			buffer.push_str(&c.to_string());
		}
		*/
		else if c == '{' {
			brkt_cnt += 1;
			buffer.push_str(&c.to_string());
		}
		/*
		else if c == '}' && brkt_cnt > 0 {
			brkt_cnt -= 1;
			buffer.push_str(&c.to_string());
		}
		else if c == '}' {
			sexp.push(buffer);
			buffer = "".to_string();
		}
		*/
		else if c == '}' {
			brkt_cnt -= 1;
			buffer.push_str(&c.to_string());
			if brkt_cnt == 0 {
				sexp.push(buffer);
				buffer = "".to_string();
			}	
		}
		else {
			buffer.push_str(&c.to_string());
		}
	}

	if !buffer.is_empty() {
		sexp.push(buffer);
	}
	//println!("{:?}", sexp);
	return sexp;
}

fn
interp(expr:LFAEDS) -> i32
{
	match expr{
		LFAEDS::Num(val) => val,
		LFAEDS::Add{l, r} => {interp(*l) + interp(*r)},
		LFAEDS::Sub{l, r} => {interp(*l) - interp(*r)},
		LFAEDS::Error => panic!("error in interp"),
	}
}
		
fn main() {
    let input1 = String::from("{+ 3 2}");
	let input2 = String::from("{+ 30 {+ 5 6}}");
	let input3 = String::from("{+ 30 {+ 5 {+ 6 7}}}");
	let input4 = String::from("{with {x 3}{+ 30 {+ 5 x}}}");
	let abs_syn = parser(&input3);
	println!("{}", abs_syn);
	println!("ans: {}", interp(abs_syn));
}
