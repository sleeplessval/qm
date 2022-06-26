use std::{
	env,
	io::{
		stdin,
		stdout,

		Write
	}
};

use evalexpr::{
	context_map,

	eval_with_context_mut,

	HashMapContext,
	Value
};
use termion::{
	color,
	style
};

mod global;
mod helper;
mod util;

pub const VERSION: &str = "0.2.0";

fn main() {
	let mut context = context_map! {
		//	globals
		"e"		=>	global::EULER,
		"phi"	=>	global::GOLDEN_RATIO,
		"pi"	=>	global::PI,
		"√2"	=>	global::ROOT_TWO,

		//	math functions
		"log"	=>	Function::new(|arg| helper::logarithm(arg)),
		"sqrt"	=>	Function::new(|arg| helper::square_root(arg)),

		//	data science functions
		"avg"	=>	Function::new(|arg| helper::average(arg)),

		//	radix functions
		"bin"	=>	Function::new(|arg| helper::binary(arg)),
		"hex"	=>	Function::new(|arg| helper::hexadecimal(arg)),
		"oct"	=>	Function::new(|arg| helper::octal(arg)),

		//	character aliases
		"ϕ"		=>	global::GOLDEN_RATIO,
		"π"		=>	global::PI,
		"√"		=>	Function::new(|arg| helper::square_root(arg))
	}.unwrap();
	let expressions: Vec<String> = env::args().skip(1).collect();
	if expressions.len() == 0 {
		println!("{}quickmaths v{}{}\n{}Interactive Mode{}", style::Bold, VERSION, style::Reset, style::Faint, style::Reset);
		loop {
			print!("> ");
			stdout().flush().unwrap();
			let mut i_line = String::new();
			let line_result = stdin().read_line(&mut i_line);
			if line_result.is_err() {
				break;
			}
			let line = i_line.trim().to_string();
			match line.as_str() {
				""		|
				"exit"	=>	break,
				_		=>	do_eval(line, &mut context)
			}
			reset();
		}
	} else {
		for expression in expressions {
			match expression.as_str() {
				"help"	=>	help_text(),
				_		=>	do_eval(expression, &mut context)
			}
		}
	}
}

fn do_eval(i_expression: String, context: &mut HashMapContext) {
	let expression = i_expression.as_str();
	let i_result = eval_with_context_mut(expression, context);
	if i_result.is_err() {
		println!("{}✕ {}{}", color::Fg(color::Red), style::Bold, expression);
		return;
	}
	let result = i_result.ok().unwrap();
	if result.is_empty() {
		println!("{}✓ {}{}", color::Fg(color::Green), style::Bold, expression);
		return;
	}
	let delimiter;
	match result {
		Value::Boolean(_bool)		=>	delimiter = "is",
		Value::String(ref _str)		=>	delimiter = "=>",
		_							=>	delimiter = "="
	}
	println!("{}{}{}{} {} {}{}", style::Faint, style::Italic, expression, style::Reset, delimiter, style::Bold, result);
}

fn reset() {
	print!("{}{}", style::Reset, color::Fg(color::Reset));
	stdout().flush().unwrap();
}

fn help_text() {
	println!("{}quickmaths v{}{}", style::Bold, crate::VERSION, style::Reset);
	println!("Valerie Wolfe <sleeplessval@gmail.com>");
	println!("A mathematical expression evaluator written in Rust.\n");
	println!("USAGE:");
	println!("\tqm [EXPRESSION]...\n");
}
