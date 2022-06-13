use evalexpr::{
	Value,

	EvalexprError
};

use crate::util;

pub fn average(arg: &Value) -> Result<Value, EvalexprError> {
	let arguments = arg.as_tuple()?;
	let count = arguments.len() as i64;
	let mut is_float = false;
	let mut total_i = 0i64;
	let mut total_f = 0f64;

	for argument in arguments {
		if let Value::Float(float) = argument {
			if !is_float {
				total_f = total_i as f64;
				is_float = true;
			}
			total_f += float;
		} else if let Value::Int(int) = argument {
			if is_float {
				total_f += int as f64;
			} else {
				total_i += int;
			}
		}
	}
	
	let result_i: i64;
	let result_f: f64;
	if !is_float {
		is_float = total_i % count == 0;
		total_f = total_i as f64;
	}

	if is_float {
		result_f = total_f / (count as f64);
		return Ok(result_f.into());
	} else {
		result_i = total_i / count;
		return Ok(result_i.into());
	}
}

pub fn binary(arg: &Value) -> Result<Value, EvalexprError> {
	if !arg.is_string() {
		let num = arg.as_int()?;
		let fmt = format!("0b{:b}", num);
		return Ok(fmt.into());
	}
	util::parse_radix("0b", 2, arg)
}

pub fn hexadecimal(arg: &Value) -> Result<Value, EvalexprError> {
	if !arg.is_string() {
		let num = arg.as_int()?;
		let fmt = format!("0x{:X}", num);
		return Ok(fmt.into());
	}
	util::parse_radix("0x", 16, arg)
}

pub fn octal(arg: &Value) -> Result<Value, EvalexprError> {
	if !arg.is_string() {
		let num = arg.as_int()?;
		let fmt = format!("{:#o}", num);
		return Ok(fmt.into());
	}
	util::parse_radix("0o", 8, arg)
}
