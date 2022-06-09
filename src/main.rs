use std::{
	env,
	io::stdin
};

use evalexpr::{
	eval_with_context_mut,

	HashMapContext,
	Value,
	ValueType
};
use termion::{
	color,
	style
};

fn main() {
	let mut context = HashMapContext::new();
	let expressions: Vec<String> = env::args().skip(1).collect();
	if expressions.len() == 0 {
		println!("{}quickmaths v0.1.1{}\n{}Interactive Mode{}", style::Bold, style::Reset, style::Faint, style::Reset);
		loop {
			let mut i_line = String::new();
			let line_result = stdin().read_line(&mut i_line);
			if line_result.is_err() {
				break;
			}
			let line = i_line.trim().to_string();
			if line.is_empty() {
				break;
			}
			let result = do_eval(line, &mut context);
			println!("{}{}{}", result.0, color::Fg(color::Reset), style::Reset);
		}
	} else {
		for expression in expressions {
			let result = do_eval(expression, &mut context);
			println!("{}{}{}", result.0, color::Fg(color::Reset), style::Reset);
		}
	}
}

fn do_eval(i_expression: String, context: &mut HashMapContext) -> (String, Option<Value>) {
	let expression = i_expression.as_str();
	let i_result = eval_with_context_mut(expression, context);
	if i_result.is_err() {
		return (format!("{}🞪 {}{}", color::Fg(color::Red), style::Bold, expression), None);
	}
	let result = i_result.ok().unwrap();
	if result.is_empty() {
		return (format!("{}✓ {}{}", color::Fg(color::Green), style::Bold, expression), None);
	}
	let delimiter;
	match result {
		Value::Boolean(_bool)		=>	delimiter = "is",
		Value::String(ref _str)	=>	delimiter = "=>",
		_							=>	delimiter = "="
	}
	return (format!("{}{}{}{} {} {}{}", style::Faint, style::Italic,	expression,	style::Reset, delimiter, style::Bold, result), Some(result));
}

