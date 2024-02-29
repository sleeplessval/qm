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
	//	build eval context
	let mut context = context_map! {
		//	globals
		"c"		=>	global::LIGHT_SPEED,
		"e"		=>	global::EULER,
		"phi"	=>	global::GOLDEN_RATIO,
		"pi"	=>	global::PI,
		"√2"	=>	global::ROOT_TWO,

		//	math functions
		"fix"	=>	Function::new(|arg| helper::fix(arg)),
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

	//	collect args and evaluate if present
	let expressions: Vec<String> = env::args().skip(1).collect();
	if expressions.len() > 0 {
		for expression in expressions {
			match expression.as_str() {
				"help"	=>	help_text(),
				_		=>	do_eval(expression, &mut context)
			}
		}
	} else {
		//	enter interactive mode if no args are given
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
	}
}

fn do_eval(i_expression: String, context: &mut HashMapContext) {
	let expression = i_expression.as_str();
	let i_result = eval_with_context_mut(expression, context);
	if i_result.is_err() {
		println!(
			"{red}✕ {bold}{expression}",
			bold = style::Bold,
			red = color::Fg(color::Red)
		);
		return;
	}
	let result = i_result.ok().unwrap();
	if result.is_empty() {
		println!(
			"{green}✓ {bold}{expression}",
			bold = style::Bold,
			green = color::Fg(color::Green)
		);
		return;
	}
	let delimiter;
	match result {
		Value::Boolean(_bool)		=>	delimiter = "is",
		Value::String(ref _str)		=>	delimiter = "=>",
		_							=>	delimiter = "="
	}
	println!(
		"{faint}{italic}{expression}{reset} {delimiter} {bold}{result}",
		bold = style::Bold,
		faint = style::Faint,
		italic = style::Italic,
		reset = style::Reset
	);
}

fn reset() {
	print!("{}{}", style::Reset, color::Fg(color::Reset));
	stdout().flush().unwrap();
}

fn help_text() {
	println!(
		"{bold}quickmaths v{version}{reset}
Valerie Wolfe <sleeplessval@gmail.com>
A mathematical expression evaluator written in Rust.

usage:
   qm [EXPRESSION]...",
		bold = style::Bold,
		reset = style::Reset,
		version = crate::VERSION
	);
}
