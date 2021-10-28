#![allow(unused)]
use std::process;
use std::fmt;
use std::env;

#[derive(Clone)]
enum LFAEDS
{
	Num(i32),
	Add{l:Box<LFAEDS>, r:Box<LFAEDS>},
	Sub{l:Box<LFAEDS>, r:Box<LFAEDS>},
	Id(String),
	Fun{param:String, body:Box<LFAEDS>},
	Dsfun{param:String, body:Box<LFAEDS>},
	App{ftn:Box<LFAEDS>, arg:Box<LFAEDS>},
	Error,
}

#[derive(Clone)]
enum DefrdSub
{
	MtSub,
	ASub{name:String, value:Box<LfaeValue>, ds:Box<DefrdSub>},
	DsSub{name:String, value:Box<LfaeValue>, ds:Box<DefrdSub>},
}

#[derive(Clone)]
enum LfaeValue
{
	NumV(i32),
	ClosureV{param:String, body:Box<LFAEDS>, ds:Box<DefrdSub>},
	ExprV{expr:Box<LFAEDS>, ds:Box<DefrdSub>, value:Box<LfaeValue>},
	False,
}

impl LfaeValue
{
	fn
	unwrap(&self) -> i32
	{
		match self{
			LfaeValue::NumV(val) => *val,
			_=> panic!("error in lfaevalue.unwrap()"),
		}
	}
	fn
	get_param(&self) -> String
	{
		match self{
			LfaeValue::ClosureV{param, body, ds} => (*param).to_string(),
			_=> panic!("error n lfaevalue.get_param()"),
		}
	}
	fn
	get_body(&self) -> LFAEDS
	{
		match self{
			LfaeValue::ClosureV{param, body, ds} => (**body).clone(),
			_=> panic!("error n lfaevalue.get_body()"),
		}
	}
	fn
	get_ds(&self) -> DefrdSub
	{
		match self{
			LfaeValue::ClosureV{param, body, ds} => (**ds).clone(),
			_=> panic!("error n lfaevalue.get_ds()"),
		}
	}
}

impl fmt::Display for LFAEDS 
{
	fn 
	fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		match self{
			LFAEDS::Num(val) => write!(f, "(num {})", val),
			LFAEDS::Add{l, r} => write!(f, "(add {} {})", l, r),
			LFAEDS::Sub{l, r} => write!(f, "(sub {} {})", l, r),
			LFAEDS::Id(name) => write!(f, "(id '{})", name),
			LFAEDS::Fun{param, body} => write!(f, "(fun '{} {})", param, body),
			LFAEDS::Dsfun{param, body} => write!(f, "(dsfun '{} {})", param, body),
			LFAEDS::App{ftn, arg} => write!(f, "(app {} {})", ftn, arg),
			LFAEDS::Error => write!(f, "error"),
		}
	}
}

impl fmt::Display for LfaeValue 
{
	fn 
	fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		match self{
			LfaeValue::NumV(val) => write!(f, "(numV {})", val),
			LfaeValue::ClosureV{param, body, ds} => write!(f, "{}", body,),
			LfaeValue::ExprV{expr, ds, value} => write!(f, "{}", value),
			LfaeValue::False => write!(f, "false"),
		}
	}
}

fn
parser(exp:&str) -> LFAEDS
{
	let sexp = split_exp(exp);
	if sexp.len() == 1 && sexp[0].parse::<i32>().is_ok() {
		return LFAEDS::Num(sexp[0].parse::<i32>().unwrap());
	} else if sexp[0].chars().nth(0).unwrap() == '+' {
		return LFAEDS::Add{l:Box::new(parser(&sexp[1])), r:Box::new(parser(&sexp[2]))};
	} else if sexp[0].chars().nth(0).unwrap() == '-' {
		return LFAEDS::Sub{l:Box::new(parser(&sexp[1])), r:Box::new(parser(&sexp[2]))};
	} else if sexp[0].eq("with") {
		return LFAEDS::App{
			ftn:Box::new(
				LFAEDS::Fun{
					param:String::from(&split_exp(&sexp[1])[0]), 
					body:Box::new(parser(&sexp[2]))
				}
			),
			arg:Box::new(parser(&split_exp(&sexp[1])[1]))
		};
	} else if sexp[0].eq("fun") {
		return LFAEDS::Fun{
			param:String::from(&split_exp(&sexp[1])[0]), 
			body:Box::new(parser(&sexp[2]))
		};
	} else if sexp[0].eq("dsfun") {
		return LFAEDS::Dsfun{
			param:String::from(&split_exp(&sexp[1])[0]), 
			body:Box::new(parser(&sexp[2]))
		};
	} else if sexp.len() == 1 && sexp[0].parse::<String>().is_ok(){
		return LFAEDS::Id(sexp[0].to_string());
	} else if sexp.len() == 2 {
		return LFAEDS::App{ftn:Box::new(parser(&sexp[0])), arg:Box::new(parser(&sexp[1]))};
	}
	return LFAEDS::Error;
}

fn
split_exp(exp:&str) -> Vec<String>
{
	//println!("{}", exp);
	if (exp.chars().nth(0).unwrap() == '{' && exp.chars().last().unwrap() != '}')
		|| (exp.chars().nth(0).unwrap() != '{' && exp.chars().last().unwrap() == '}') {
		//println!("input error");
		//process::exit(1);
		println!("__{}__", exp);
		panic!("input error");
	}
	let (mut start_ind, mut end_ind) = (0, exp.len());
	if exp.chars().nth(0).unwrap() == '{' {
		start_ind = 1;
		end_ind -= 1;
	}
	let example_code = &exp[start_ind..end_ind];
	let (mut brkt_cnt, mut buffer) = (0, "".to_string());
	let mut sexp:Vec<String> = Vec::new();
	for c in example_code.chars() {
		if c == ' ' && brkt_cnt == 0 {
			if !buffer.is_empty() {
				sexp.push(buffer);
				buffer = "".to_string();
			}
		} else if c == '{' {
			brkt_cnt += 1;
			buffer.push_str(&c.to_string());
		} else if c == '}' {
			brkt_cnt -= 1;
			buffer.push_str(&c.to_string());
			if brkt_cnt == 0 {
				sexp.push(buffer);
				buffer = "".to_string();
			}	
		} else {
			buffer.push_str(&c.to_string());
		}
	}
	if !buffer.is_empty() {
		sexp.push(buffer);
	}
	//println!("sexp:: {:?}", sexp);
	return sexp;
}

fn
strict(v:LfaeValue) -> LfaeValue
{
	match v {
		LfaeValue::ExprV{expr, ds, mut value} => {
			match *value {
				LfaeValue::False => {
					let v = strict(interp(&*expr, &ds));
					*value = v.clone();
					v
				},
				_=> *value,
			}
		}
		_=> v,
	}
}

fn
look_up(id:String, ds_param:&DefrdSub) ->LfaeValue
{
	match ds_param{
		DefrdSub::MtSub => panic!("error 'lookup free identifier"),
		DefrdSub::ASub{name, value, ds} =>{
			if id.eq(name){
				strict((**value).clone())
			} else{
				look_up(id, &*ds)
			}
		}
		DefrdSub::DsSub{name, value, ds} => {
			if id.eq(name){
				strict((**value).clone())
			} else{
				look_up(id, &*ds)
			}
		}
	}
}
fn
interp(expr:&LFAEDS, ds:&DefrdSub) -> LfaeValue
{
	match expr{
		LFAEDS::Num(val) => LfaeValue::NumV(*val),
		LFAEDS::Add{l, r} => LfaeValue::NumV(interp(&*l, &ds).unwrap() + interp(&*r, &ds).unwrap()),
		LFAEDS::Sub{l, r} => LfaeValue::NumV(interp(&*l, &ds).unwrap() - interp(&*r, &ds).unwrap()),
		LFAEDS::Id(name) => look_up(name.to_string(), &ds),
		LFAEDS::Id(name) => look_up(name.to_string(), &ds),
		LFAEDS::Fun{param, body} => { LfaeValue::ClosureV{
										param:param.to_string(), 
										body:Box::new((**body).clone()), 
										ds:Box::new((*ds).clone()) }},
		LFAEDS::Dsfun{param, body} => { LfaeValue::ClosureV{
										param:param.to_string(), 
										body:Box::new((**body).clone()), 
										ds:Box::new((*ds).clone()) }},

		LFAEDS::App{ftn, arg} => {
			let f_val = strict(interp(&*ftn, &ds));
			let a_val = LfaeValue::ExprV{
								expr:Box::new((**arg).clone()), 
								ds:Box::new((*ds).clone()), 
								value:Box::new(LfaeValue::False)
							};
			interp(&f_val.get_body(), 
					&DefrdSub::ASub{
						name:f_val.get_param(), 
						value:Box::new(a_val), 
						ds:Box::new(f_val.get_ds()) })
		},
		LFAEDS::Error => panic!("error in interp"),
	}
}
		
fn 
main() 
{
	let args:Vec<String> = env::args().collect();
	if args.len() == 3 && args[1].eq("-p"){
		println!("{}", parser(&args[2]));
	}
	else if args.len() == 2 {
		println!("{}", interp(&parser(&args[1]), &DefrdSub::MtSub));
	}
	else {
		println!("input error");
	}
}
